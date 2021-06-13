use {
    crate::{
        screens::manager::Message, styles, widgets::clickable::Clickable, DEF_PADDING,
        SMALL_PADDING, SMALL_TEXT,
    },
    iced::{button, container, Align, Checkbox, Color, Column, Element, Length, Text},
};

#[derive(Debug, Clone)]
pub struct Item {
    click: button::State,
    pub name: String,
    pub price: u32,
    pub available: bool,
}

impl Item {
    pub fn new(name: &str, price: u32, available: bool) -> Item {
        Self {
            click: button::State::new(),
            name: String::from(name),
            price,
            available,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let name = self.name.clone();
        let clone = self.clone();
        Clickable::new(
            &mut self.click,
            container::Container::new(
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
            .style(styles::Container {
                text_color: Some(Color::BLACK),
                background: None,
                border_radius: 2f32,
                border_width: 2f32,
                border_color: Color::BLACK,
            }),
        )
        .on_press(Message::EditItem(clone))
        .width(Length::Fill)
        .into()
    }
}
