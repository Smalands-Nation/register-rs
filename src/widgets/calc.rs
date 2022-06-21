use {
    super::{Grid, SquareButton},
    crate::{
        icons::Icon,
        styles::{DEF_PADDING, DEF_TEXT, SQUARE_BUTTON},
    },
    iced::{
        alignment::{Alignment, Horizontal},
        pure::{
            widget::{Column, Row, Rule, Space, Text},
            Element,
        },
        Length,
    },
};

pub struct Calc(pub u32, u32);

#[derive(Debug, Clone)]
pub enum Message {
    Clear,
    Update(u32),
    Save,
}

impl Calc {
    pub fn new() -> Self {
        Self(1, 0)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Clear if self.1 == 0 => self.0 = 1,
            Message::Clear if self.1 != 0 => self.1 = 0,
            Message::Update(v) if (v, self.1) != (0, 0) => {
                self.1 = match self.1 * 10 + v {
                    0 => 1,
                    v @ 1..=100 => v,
                    _ => 100,
                }
            }
            Message::Save if self.1 != 0 => {
                self.0 = self.1;
                self.1 = 0;
            }
            _ => (),
        };
    }

    pub fn view(&self) -> Element<Message> {
        Column::new()
            .align_items(Alignment::Center)
            .push(
                Row::new()
                    .push(
                        Text::new(format!("{:>3}x", self.0)).horizontal_alignment(Horizontal::Left),
                    )
                    .push(Rule::vertical(DEF_PADDING))
                    .push(
                        Text::new(if self.1 != 0 {
                            format!("{}", self.1)
                        } else {
                            String::new()
                        })
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Right),
                    )
                    .height(Length::Units(DEF_TEXT))
                    .width(Length::Units(SQUARE_BUTTON * 3 + DEF_PADDING * 2)),
            )
            .push(Space::with_height(Length::Units(DEF_PADDING)))
            .push(
                Grid::with_children(
                    4,
                    3,
                    (0..12)
                        .map(|i| {
                            match i {
                                0..=8 => SquareButton::new(Text::new(format!("{}", i + 1)))
                                    .on_press(Message::Update(i as u32 + 1)),
                                9 => SquareButton::new(Text::new("c")).on_press(Message::Clear),
                                10 => {
                                    SquareButton::new(Text::new("0")).on_press(Message::Update(0))
                                }
                                _ => SquareButton::new(Icon::Cross).on_press(Message::Save),
                            }
                            .into()
                        })
                        .collect(),
                )
                .spacing(DEF_PADDING),
            )
            .into()
    }
}
