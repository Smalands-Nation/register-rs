use {
    crate::styles::{BORDERED, DEF_PADDING, SMALL_PADDING, SMALL_TEXT},
    iced::{Column, Container, Element, HorizontalAlignment, Length, Row, Text},
    serde_derive::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    Regular { name: String, price: i32, num: i32 },
    Special { name: String, price: i32 },
}

impl Item {
    pub fn name(&self) -> String {
        match self {
            Self::Regular { name, .. } | Self::Special { name, .. } => name.into(),
        }
    }

    pub fn price(&self) -> i32 {
        match self {
            Self::Regular { price, .. } | Self::Special { price, .. } => *price,
        }
    }

    pub fn num(&self) -> i32 {
        match self {
            Self::Regular { num, .. } | Self::Special { price: num @ _, .. } => *num,
        }
    }

    pub fn price_total(&self) -> i32 {
        match self {
            Self::Regular { price, num, .. } => *num * *price,
            Self::Special { price, .. } => *price,
        }
    }

    pub fn view<'a, M>(&'a mut self) -> Element<M>
    where
        M: 'a + Clone,
    {
        Container::new(
            Column::new()
                .spacing(SMALL_PADDING)
                .push(Text::new(self.name().as_str()))
                .push(match self {
                    Self::Regular { price, num, .. } => Row::new()
                        .push(Text::new(format!("{}x{} kr", num, price)).size(SMALL_TEXT))
                        .push(
                            Text::new(format!("{} kr", *num * *price))
                                .size(SMALL_TEXT)
                                .width(Length::Fill)
                                .horizontal_alignment(HorizontalAlignment::Right),
                        ),
                    Self::Special { price, .. } => Row::new().push(
                        Text::new(format!("{} kr", price))
                            .size(SMALL_TEXT)
                            .width(Length::Fill)
                            .horizontal_alignment(HorizontalAlignment::Right),
                    ),
                }),
        )
        .padding(DEF_PADDING)
        .width(Length::Fill)
        .style(BORDERED)
        .into()
    }
}
