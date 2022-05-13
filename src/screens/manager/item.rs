use {
    super::Message,
    crate::{
        styles::{BORDERED, DEF_PADDING, SMALL_PADDING, SMALL_TEXT},
        widgets::Clickable,
    },
    iced::{
        pure::{
            widget::{Checkbox, Column, Container, Text},
            Element,
        },
        Length,
    },
};

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub available: bool,
}

impl Item {
    pub fn new(name: &str, price: u32, available: bool) -> Item {
        Self {
            name: String::from(name),
            price,
            available,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let name = self.name.clone();
        let clone = self.clone();
        Clickable::new(
            Container::new(
                Column::new()
                    .spacing(SMALL_PADDING)
                    .push(Text::new(self.name.as_str()))
                    .push(Text::new(format!("{} kr", self.price)).size(SMALL_TEXT))
                    .push(
                        Checkbox::new(self.available, "I Lager", move |b| {
                            Message::ToggleItem(name.clone(), b)
                        })
                        .text_size(SMALL_TEXT),
                    ),
            )
            .padding(DEF_PADDING)
            .width(Length::Fill)
            .style(BORDERED),
        )
        .on_press(Message::EditItem(clone))
        .width(Length::Fill)
        .into()
    }
}
