use {
    crate::{helper::Style, screens::manager::Message, DEF_PADDING, SMALL_PADDING, SMALL_TEXT},
    iced::{container, Align, Checkbox, Color, Column, Element, Length, Text},
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
        let clone = self.name.clone();
        container::Container::new(
            Column::new()
                .spacing(SMALL_PADDING)
                .push(Text::new(self.name.as_str()))
                .push(Text::new(format!("{:.2} kr", self.price as f32 / 100.0)).size(SMALL_TEXT))
                .push(
                    Checkbox::new(self.available, "I Lager", move |b| {
                        Message::ToggleItem(clone.clone(), b)
                    })
                    .text_size(SMALL_TEXT),
                ),
        )
        .padding(DEF_PADDING)
        .width(Length::Fill)
        .style(Style(container::Style {
            text_color: Some(Color::BLACK),
            background: None,
            border_radius: 2f32,
            border_width: 2f32,
            border_color: Color::BLACK,
        }))
        .into()
    }
}
