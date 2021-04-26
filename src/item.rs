use {
    crate::{helper::Clickable, Message},
    iced::{
        button, container, Align, Button, Color, Column, Element, HorizontalAlignment, Length, Row,
        Text,
    },
    std::{
        cmp::{Eq, PartialEq},
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    },
};

#[derive(Clone, Debug, Eq)]
pub enum Item {
    OnMenu(String, u32, button::State),
    Sold(String, u32, u32),
    Invisible,
}

impl Item {
    pub fn new(name: &str, price: u32) -> Self {
        Self::OnMenu(name.into(), price, button::State::new())
    }

    pub fn sell(self, num: u32) -> Self {
        match self {
            Self::OnMenu(name, price, _) | Self::Sold(name, price, _) => {
                Self::Sold(name, price, num)
            }
            Self::Invisible => self,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        match self {
            Self::OnMenu(name, price, state) => Button::new(
                state,
                container::Container::new(
                    Column::new()
                        .align_items(Align::Center)
                        .spacing(5)
                        .push(Text::new(name.as_str()))
                        .push(Text::new(format!("{:.2} kr", *price as f32 / 100.0)).size(20)),
                )
                .padding(10)
                .width(Length::Fill)
                .style(Style(container::Style {
                    text_color: Some(Color::BLACK),
                    background: None,
                    border_radius: 2f32,
                    border_width: 2f32,
                    border_color: Color::BLACK,
                })),
            )
            .width(Length::Fill)
            .style(Clickable)
            .on_press(Message::Sell(Self::Sold(name.to_string(), *price, 1)))
            .into(),
            Self::Sold(name, price, num) => container::Container::new(
                Column::new()
                    .align_items(Align::Center)
                    .spacing(5)
                    .push(Text::new(name.as_str()))
                    .push(
                        Row::new()
                            .push(
                                Text::new(format!("{}x{:.2} kr", *num, *price as f32 / 100.0))
                                    .size(20),
                            )
                            .push(
                                Text::new(format!("{:.2} kr", (*num * *price) as f32 / 100.0))
                                    .size(20)
                                    .width(Length::Fill)
                                    .horizontal_alignment(HorizontalAlignment::Right),
                            ),
                    ),
            )
            .padding(10)
            .width(Length::Fill)
            .style(Style(container::Style {
                text_color: Some(Color::BLACK),
                background: None,
                border_radius: 2f32,
                border_width: 2f32,
                border_color: Color::BLACK,
            }))
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
                state.write_u32(*price);
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

struct Style(container::Style);

impl container::StyleSheet for Style {
    fn style(&self) -> container::Style {
        self.0
    }
}
