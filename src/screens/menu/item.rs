use {
    super::Message,
    crate::{
        screens::transactions,
        styles::{BORDERED, DEF_PADDING, SMALL_PADDING, SMALL_TEXT},
        widgets::Clickable,
    },
    iced::{button, Column, Container, Element, Length, Text},
};

#[derive(Debug, Clone)]
pub struct Item {
    state: button::State,
    pub name: String,
    pub price: i32,
    pub special: bool,
}

impl Item {
    pub fn new(name: &str, price: i32, special: bool) -> Self {
        Self {
            state: button::State::new(),
            name: String::from(name),
            price,
            special,
        }
    }

    pub fn sell(&self, num: i32) -> transactions::Item {
        if self.special {
            transactions::Item::Special {
                name: self.name.clone(),
                price: self.price * num,
            }
        } else {
            transactions::Item::Regular {
                name: self.name.clone(),
                price: self.price,
                num,
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let clone = self.clone();
        Clickable::new(
            &mut self.state,
            Container::new(
                Column::new()
                    .spacing(SMALL_PADDING)
                    .push(Text::new(self.name.as_str()))
                    .push(Text::new(format!("{} kr", self.price)).size(SMALL_TEXT)),
            )
            .padding(DEF_PADDING)
            .width(Length::Fill)
            .style(BORDERED),
        )
        .width(Length::Fill)
        .on_press(Message::SellItem(clone))
        .into()
    }
}
