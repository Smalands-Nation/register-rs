use {
    crate::{grid::Grid, BIG_TEXT, DEF_PADDING, DEF_TEXT},
    iced::{
        button, Align, Button, Column, Element, HorizontalAlignment, Length, Row, Rule, Space, Text,
    },
};

pub struct Calc(pub u32, u32, [button::State; 12]);

#[derive(Debug, Clone)]
pub enum Message {
    Clear,
    Update(u32),
    Save,
}

impl Calc {
    pub fn new() -> Self {
        Self(1, 0, [button::State::new(); 12])
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

    pub fn view(&mut self) -> Element<Message> {
        let sq: u32 = 15 + BIG_TEXT as u32;
        Column::new()
            .align_items(Align::Center)
            .push(
                Row::new()
                    .push(
                        Text::new(format!("{:>3}x", self.0))
                            .horizontal_alignment(HorizontalAlignment::Left),
                    )
                    .push(Rule::vertical(DEF_PADDING))
                    .push(
                        Text::new(if self.1 != 0 {
                            format!("{}", self.1)
                        } else {
                            String::new()
                        })
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Right),
                    )
                    .max_height(DEF_TEXT.into())
                    .max_width(sq * 3 + (DEF_PADDING as u32) * 2),
            )
            .push(Space::with_height(Length::Units(DEF_PADDING)))
            .push(
                Grid::with_children(
                    4,
                    3,
                    self.2
                        .iter_mut()
                        .enumerate()
                        .map(move |(i, st)| {
                            match i {
                                0..=8 => {
                                    Button::new(st, Text::new(format!("{}", i + 1)).size(BIG_TEXT))
                                        .on_press(Message::Update(i as u32 + 1))
                                }
                                9 => Button::new(st, Text::new("c").size(BIG_TEXT))
                                    .on_press(Message::Clear),
                                10 => Button::new(st, Text::new("0").size(BIG_TEXT))
                                    .on_press(Message::Update(0)),
                                _ => Button::new(st, Text::new("x").size(BIG_TEXT))
                                    .on_press(Message::Save),
                            }
                            .min_height(sq)
                            .min_width(sq)
                            .into()
                        })
                        .collect(),
                )
                .spacing(DEF_PADDING),
            )
            .into()
    }
}
