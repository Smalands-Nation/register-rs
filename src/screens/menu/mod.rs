pub use {
    crate::{
        calc, calc::Calc, error::Result, grid::Grid, icons::Icon, BIG_TEXT, DEF_PADDING, DEF_TEXT,
    },
    iced::{
        button, scrollable, window, Application, Button, Checkbox, Clipboard, Column, Command,
        Container, Element, Font, HorizontalAlignment, Length, Row, Rule, Scrollable, Settings,
        Space, Text,
    },
    item::Item,
    rusqlite::Connection,
    std::collections::HashMap,
};

pub mod item;

pub struct Menu {
    calc: Calc,
    menu: Vec<Item>,
    reciept: HashMap<Item, Item>,
    scroll: scrollable::State,
    clear: button::State,
    total: i32,
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
    ClearReciept,
    TogglePrint(bool),
    Sell(Payment),
}

impl Menu {
    pub fn new(mut menu: Vec<Item>) -> Self {
        menu.append(&mut vec![
            Item::new_special("Special", 100),
            Item::new_special("Rabatt", -100),
        ]);
        menu.append(&mut vec![Item::Invisible; 3 - menu.len() % 3]);

        Self {
            calc: Calc::new(),
            menu,
            reciept: HashMap::new(),
            scroll: scrollable::State::new(),
            clear: button::State::new(),
            total: 0,
            print: false,
            cash: button::State::new(),
            swish: button::State::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::None => (),
            Message::Calc(m) => self.calc.update(m),
            Message::ClearReciept => {
                self.reciept = HashMap::new();
                self.total = 0;
            }
            Message::SellItem(i) => {
                let i = i.sell(self.calc.0 as i32);
                match self.reciept.get_mut(&i) {
                    Some(it) => {
                        *it = match (i, it.clone()) {
                            (Item::Sold(n1, p1, x1), Item::Sold(n2, p2, x2))
                                if n1 == n2 && p1 == p2 =>
                            {
                                Item::Sold(n1, p1, x1 + x2)
                            }
                            (Item::SoldSpecial(n1, p1), Item::SoldSpecial(n2, p2)) if n1 == n2 => {
                                Item::SoldSpecial(n1, p1 + p2)
                            }
                            (_, it @ _) => it,
                        };
                    }
                    None => {
                        self.reciept.insert(i.clone(), i);
                    }
                }
                self.total = self.reciept.values_mut().fold(0i32, |acc, n| {
                    acc + match n {
                        Item::Sold(_, price, num) => *price * *num,
                        Item::SoldSpecial(_, price) => *price,
                        _ => 0,
                    }
                });
                self.calc.update(calc::Message::Clear);
            }
            Message::TogglePrint(b) => self.print = b,
            Message::Sell(_p) => {
                let r: Vec<Item> = self.reciept.values().map(|v| v.clone()).collect();
                self.update(Message::ClearReciept);
                println!("{:?}", r);
            }
        };
        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
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
                Row::new()
                    .push(Text::new("Kvitto").size(BIG_TEXT))
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Button::new(&mut self.clear, Text::from(Icon::Trash))
                            .on_press(Message::ClearReciept),
                    )
                    .into(),
                self.reciept
                    .values_mut()
                    .fold(
                        Scrollable::new(&mut self.scroll).spacing(DEF_PADDING),
                        |c, i| c.push(i.view()),
                    )
                    .height(Length::Fill)
                    .into(),
                Text::new(format!("Total: {:.2}kr", self.total as f32 / 100.0)).into(),
                Checkbox::new(self.print, "Printa kvitto", |b| Message::TogglePrint(b)).into(),
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
