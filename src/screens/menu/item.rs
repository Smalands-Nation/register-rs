use {
    crate::{
        screens::menu::Message, styles, widgets::clickable::Clickable, DEF_PADDING, SMALL_PADDING,
        SMALL_TEXT,
    },
    iced::{
        button, container, Align, Button, Color, Column, Container, Element, HorizontalAlignment,
        Length, Row, Text,
    },
    std::{
        cmp::{Eq, PartialEq},
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    },
};

#[derive(Clone, Debug, Eq)]
pub enum Item {
    OnMenu(String, i32, button::State),
    OnMenuSpecial(String, i32, button::State),
    Sold(String, i32, i32),
    SoldSpecial(String, i32),
    Invisible,
}

impl Item {
    pub fn new(name: &str, price: i32) -> Self {
        Self::OnMenu(name.into(), price, button::State::new())
    }

    pub fn new_special(name: &str, price: i32) -> Self {
        Self::OnMenuSpecial(name.into(), price, button::State::new())
    }

    pub fn sell(self, num: i32) -> Self {
        match self {
            Self::OnMenu(name, price, _) | Self::Sold(name, price, _) => {
                Self::Sold(name, price, num)
            }
            Self::OnMenuSpecial(name, price, _) | Self::SoldSpecial(name, price) => {
                Self::SoldSpecial(name, price * num)
            }
            Self::Invisible => self,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let clone = self.clone();
        match self {
            Self::OnMenu(name, price, state) | Self::OnMenuSpecial(name, price, state) => {
                Clickable::new(
                    state,
                    Container::new(
                        Column::new()
                            .spacing(SMALL_PADDING)
                            .push(Text::new(name.as_str()))
                            .push(Text::new(format!("{} kr", price)).size(SMALL_TEXT)),
                    )
                    .padding(DEF_PADDING)
                    .width(Length::Fill)
                    .style(styles::Container {
                        text_color: Some(Color::BLACK),
                        background: None,
                        border_radius: 2f32,
                        border_width: 2f32,
                        border_color: Color::BLACK,
                    }),
                )
                .width(Length::Fill)
                .on_press(Message::SellItem(clone))
                .into()
            }
            Self::Sold(name, price, num) => Container::new(
                Column::new()
                    .spacing(SMALL_PADDING)
                    .push(Text::new(name.as_str()))
                    .push(
                        Row::new()
                            .push(Text::new(format!("{}x{} kr", num, price)).size(SMALL_TEXT))
                            .push(
                                Text::new(format!("{} kr", *num * *price))
                                    .size(SMALL_TEXT)
                                    .width(Length::Fill)
                                    .horizontal_alignment(HorizontalAlignment::Right),
                            ),
                    ),
            )
            .padding(DEF_PADDING)
            .width(Length::Fill)
            .style(styles::Container {
                text_color: Some(Color::BLACK),
                background: None,
                border_radius: 2f32,
                border_width: 2f32,
                border_color: Color::BLACK,
            })
            .into(),
            Self::SoldSpecial(name, price) => Container::new(
                Column::new()
                    .spacing(SMALL_PADDING)
                    .push(Text::new(name.as_str()))
                    .push(
                        Text::new(format!("{} kr", price))
                            .size(SMALL_TEXT)
                            .width(Length::Fill)
                            .horizontal_alignment(HorizontalAlignment::Right),
                    ),
            )
            .padding(DEF_PADDING)
            .width(Length::Fill)
            .style(styles::Container {
                text_color: Some(Color::BLACK),
                background: None,
                border_radius: 2f32,
                border_width: 2f32,
                border_color: Color::BLACK,
            })
            .into(),
            Self::Invisible => Column::new().width(Length::Fill).into(),
        }
    }
}

impl Hash for Item {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self {
            Self::OnMenu(name, price, _) | Self::Sold(name, price, _) => {
                state.write(name.as_bytes());
                state.write_i32(*price);
            }
            Self::OnMenuSpecial(name, _, _) | Self::SoldSpecial(name, _) => {
                let mut h_name = name.clone();
                h_name.push_str("__Special__");
                state.write(h_name.as_bytes())
            }
            Self::Invisible => {
                state.write(b"Item::Invisible");
            }
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        self.hash(&mut h1);
        other.hash(&mut h2);
        h1.finish() == h2.finish()
    }
}
