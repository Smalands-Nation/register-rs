use {
    calc::Calc,
    grid::Grid,
    iced::{
        button, scrollable, window, Application, Button, Checkbox, Clipboard, Column, Command,
        Container, Element, Font, Length, Row, Rule, Scrollable, Settings, Space, Text,
    },
    item::Item,
    std::collections::HashMap,
};

mod calc;
mod grid;
mod helper;
mod item;

const BIG_TEXT: u16 = 45;
const DEF_TEXT: u16 = 35;
const SMALL_TEXT: u16 = 20;

const DEF_PADDING: u16 = 10;
const SMALL_PADDING: u16 = 5;

const FONT: Font = Font::External {
    name: "IBM Plex Mono",
    bytes: include_bytes!("../resources/IBMPlexMono-Regular.ttf"),
};

pub fn main() -> iced::Result {
    Screen::run(Settings {
        window: window::Settings {
            min_size: Some((1300, 600)),
            ..window::Settings::default()
        },
        default_font: match FONT {
            Font::External { bytes, .. } => Some(bytes),
            _ => None,
        },
        default_text_size: DEF_TEXT,
        ..Settings::default()
    })
}

pub struct Screen {
    calc: Calc,
    menu: Vec<Item>,
    reciept: HashMap<Item, Item>,
    scroll: scrollable::State,
    print: bool,
    cash: button::State,
    swish: button::State,
}

#[derive(Debug, Clone)]
pub enum Payment {
    Cash,
    Swish,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    Calc(calc::Message),
    SellItem(Item),
    TogglePrint(bool),
    Sell(Payment),
}

impl Application for Screen {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Message>) {
        let mut menu = vec![Item::new("Åbro", 1500); 4]; //Db request here
        menu.push(Item::new("Xide", 1500));
        menu.append(&mut vec![Item::Invisible; 3 - menu.len() % 3]);

        (
            Self {
                calc: Calc::new(),
                menu,
                reciept: HashMap::new(),
                scroll: scrollable::State::new(),
                print: false,
                cash: button::State::new(),
                swish: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Menu")
    }

    fn update(&mut self, message: Message, _: &mut Clipboard) -> Command<Message> {
        match message {
            Message::None => (),
            Message::Calc(m) => self.calc.update(m),
            Message::SellItem(i) => {
                match self.reciept.get_mut(&i) {
                    Some(it) => {
                        *it = match (i, it.clone()) {
                            (Item::Sold(n1, p1, x1), Item::Sold(n2, p2, x2))
                                if n1 == n2 && p1 == p2 =>
                            {
                                Item::Sold(n1, p1, x1 + x2)
                            }
                            (_, it @ _) => it,
                        };
                    }
                    None => {
                        self.reciept.insert(i.clone(), i);
                    }
                }
                self.calc.update(calc::Message::Clear);
            }
            Message::TogglePrint(b) => self.print = b,
            Message::Sell(_p) => {
                let r: Vec<Item> = self.reciept.values().map(|v| v.clone()).collect();
                self.reciept = HashMap::new();
                println!("{:?}", r);
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Row::with_children(vec![
            Container::new(self.calc.view().map(Message::Calc))
                .padding(DEF_PADDING)
                .center_x()
                .center_y()
                .width(Length::FillPortion(3))
                .height(Length::Fill)
                .into(),
            Rule::vertical(DEF_PADDING).into(),
            Grid::with_children(
                self.menu.len() as u32 / 3,
                3,
                self.menu.iter_mut().map(|i| i.view()).collect(),
            )
            .width(Length::FillPortion(8))
            .spacing(DEF_PADDING)
            .padding(DEF_PADDING)
            .into(),
            Rule::vertical(DEF_PADDING).into(),
            Column::with_children(vec![
                Text::new("Kvitto").size(BIG_TEXT).into(),
                self.reciept
                    .values_mut()
                    .fold(
                        Scrollable::new(&mut self.scroll).spacing(DEF_PADDING),
                        |c, i| c.push(i.view()),
                    )
                    .height(Length::Fill)
                    .into(),
                Checkbox::new(self.print, "Kivtto", |b| Message::TogglePrint(b)).into(),
                Button::new(&mut self.cash, Text::new("Kontant").size(BIG_TEXT))
                    .on_press(Message::Sell(Payment::Cash))
                    .padding(DEF_PADDING)
                    .width(Length::Fill)
                    .into(),
                Button::new(&mut self.swish, Text::new("Swish").size(BIG_TEXT))
                    .on_press(Message::Sell(Payment::Swish))
                    .padding(DEF_PADDING)
                    .width(Length::Fill)
                    .into(),
            ])
            .width(Length::FillPortion(3))
            .spacing(DEF_PADDING)
            .padding(DEF_PADDING)
            .into(),
        ])
        .into()
    }
}
