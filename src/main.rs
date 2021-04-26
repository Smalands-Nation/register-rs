use {
    calc::Calc,
    grid::Grid,
    iced::{
        scrollable, window, Application, Clipboard, Column, Command, Container, Element, Font,
        Length, Row, Rule, Scrollable, Settings, Space, Text,
    },
    item::Item,
    std::collections::HashMap,
};

mod calc;
mod grid;
mod helper;
mod item;

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
        default_text_size: 35,
        ..Settings::default()
    })
}

pub struct Screen {
    calc: Calc,
    menu: Vec<Item>,
    reciept: HashMap<Item, Item>,
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Calc(calc::Message),
    Sell(Item),
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
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Menu")
    }

    fn update(&mut self, message: Message, _: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Calc(m) => self.calc.update(m),
            Message::Sell(i) => {
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
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Row::with_children(vec![
            Container::new(self.calc.view().map(Message::Calc))
                .padding(10)
                .center_x()
                .center_y()
                .width(Length::FillPortion(3))
                .height(Length::Fill)
                .into(),
            Rule::vertical(10).into(),
            Grid::with_children(
                self.menu.len() as u32 / 3,
                3,
                self.menu.iter_mut().map(|i| i.view()).collect(),
            )
            .width(Length::FillPortion(8))
            .spacing(10)
            .padding(10)
            .into(),
            Rule::vertical(10).into(),
            Column::with_children(vec![
                Text::new("Kvitto").size(45).into(),
                self.reciept
                    .values_mut()
                    .fold(Scrollable::new(&mut self.scroll).spacing(10), |c, i| {
                        c.push(i.view())
                    })
                    .into(),
            ])
            .width(Length::FillPortion(3))
            .spacing(10)
            .padding(10)
            .into(),
        ])
        .into()
    }
}
