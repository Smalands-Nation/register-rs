use {
    super::{db, Screen},
    crate::{
        icons::Icon,
        payment::Payment,
        print,
        styles::{BORDERED, DEF_PADDING, RECIEPT_WIDTH},
        widgets::{Clickable, Reciept, SquareButton},
    },
    chrono::{DateTime, Local},
    iced::{button, Column, Command, Container, Element, Length, Row, Rule, Space, Text},
    indexmap::IndexMap,
    rusqlite::params,
    std::future,
};

pub mod item;
pub use item::Item;

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    Init(IndexMap<DateTime<Local>, Reciept<Message>>),
    Append(IndexMap<DateTime<Local>, Reciept<Message>>),
    ScrollLeft,
    ScrollRight,
    Select(DateTime<Local>),
    Deselect,
    Print,
}

pub struct Transactions {
    left: button::State,
    right: button::State,
    deselect: button::State,
    print: button::State,
    reciepts: IndexMap<DateTime<Local>, Reciept<Message>>,
    selected: Option<(DateTime<Local>, Reciept<Message>)>,
    offset: usize,
}

impl Screen for Transactions {
    type InMessage = Message;
    type ExMessage = super::Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                left: button::State::new(),
                right: button::State::new(),
                deselect: button::State::new(),
                print: button::State::new(),
                reciepts: IndexMap::new(),
                selected: None,
                offset: 0,
            },
            future::ready(Message::Refresh.into()).into(),
        )
    }

    fn update(&mut self, msg: Message) -> Command<Self::ExMessage> {
        match msg {
            Message::Refresh => {
                return db(|con| {
                    Ok(Message::Init(
                            con.lock()
                                .unwrap()
                                .prepare("SELECT * FROM reciepts where time > date('now','-1 day') ORDER BY time DESC")? //All reciepts from last 24h, potentially bad for performance
                                .query_map(params![], |row| {
                                    Ok((
                                        //God hates me so all of these are type annotated
                                        row.get::<usize,DateTime<Local>>(0)?,
                                        serde_json::de::from_str(
                                            row.get::<usize, String>(1)?.as_str(),
                                        )
                                        .unwrap(),
                                        row.get::<usize, i32>(2)?,
                                        match row.get::<usize, String>(3)?.as_str() {
                                            "Cash" => Payment::Cash,
                                            _ => Payment::Swish,
                                        },
                                    ))
                                })?
                                .fold(IndexMap::new(), |mut hm, row| {
                                    let row = row.unwrap();
                                    hm.insert(row.0.clone(), Reciept::new_from(row.1, row.2, row.3).on_press(Message::Select(row.0)));
                                    hm
                                }),
                        ).into())
                });
            }
            Message::Init(map) => self.reciepts = map,
            Message::Append(map) => self.reciepts.extend(map),
            Message::ScrollLeft if self.offset > 0 => self.offset -= 1,
            Message::ScrollRight
                if self.reciepts.len() != 0 && self.offset < (self.reciepts.len() - 1) / 3 =>
            {
                self.offset += 1
            }
            Message::Select(time) => {
                self.selected = self
                    .reciepts
                    .get_key_value(&time)
                    .map(|(k, v)| (k.clone(), v.clone()));
            }
            Message::Deselect => self.selected = None,
            Message::Print => {
                if let Some((time, reciept)) = &self.selected {
                    return Command::perform(
                        print::print((**reciept).clone(), time.clone()),
                        |r| match r {
                            Ok(_) => Message::Deselect.into(),
                            Err(e) => super::Message::Error(e),
                        },
                    );
                }
            }
            _ => (),
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::ExMessage> {
        Element::<Self::InMessage>::from(
            Row::new()
                .push(
                    Container::new(
                        self.reciepts
                            .values_mut()
                            .skip(self.offset * 3)
                            .take(3)
                            .fold(
                                Row::new().push(
                                    Clickable::new(
                                        &mut self.left,
                                        Container::new(Text::from(Icon::Left))
                                            .width(Length::Fill)
                                            .height(Length::Fill)
                                            .center_x()
                                            .center_y(),
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .on_press(Message::ScrollLeft),
                                ),
                                |row, rec| {
                                    row.push(
                                        Container::new(rec.view())
                                            .padding(DEF_PADDING)
                                            .style(BORDERED),
                                    )
                                },
                            )
                            .push(
                                Clickable::new(
                                    &mut self.right,
                                    Container::new(Text::from(Icon::Right))
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .center_x()
                                        .center_y(),
                                )
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .on_press(Message::ScrollRight),
                            )
                            .padding(DEF_PADDING)
                            .spacing(DEF_PADDING),
                    )
                    .center_x()
                    .width(Length::Fill),
                )
                .push(Rule::vertical(DEF_PADDING))
                .push(
                    match &mut self.selected {
                        Some((_, rec)) => Column::new().push(rec.view()),
                        None => Column::new()
                            .push(Space::new(Length::Units(RECIEPT_WIDTH), Length::Fill)),
                    }
                    .push(
                        Row::new()
                            .push(
                                SquareButton::new(&mut self.deselect, Text::from(Icon::Cross))
                                    .on_press(Message::Deselect),
                            )
                            .push(Space::with_width(Length::Fill))
                            .push(
                                SquareButton::new(&mut self.print, Text::from(Icon::Print))
                                    .on_press(Message::Print),
                            )
                            .padding(DEF_PADDING)
                            .spacing(DEF_PADDING),
                    )
                    .padding(DEF_PADDING)
                    .width(Length::Units(RECIEPT_WIDTH)),
                ),
        )
        .map(Self::ExMessage::Transactions)
    }
}