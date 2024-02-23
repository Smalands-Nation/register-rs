use {
    super::{Category, ItemKind},
    crate::{
        theme::{Container, DEF_PADDING, RECEIPT_WIDTH, SMALL_PADDING},
        widgets::{column, row, SMALL_TEXT},
    },
    iced::{
        alignment::Horizontal,
        widget::{Button, Checkbox, Component, Text},
        Element, Length,
    },
};

pub struct Item<'a, M> {
    name: String,
    price: i32,
    category: Category,
    kind: ItemKind,
    on_press: Option<M>,
    on_toggle: Option<Box<dyn Fn(bool) -> M + 'a>>,
    width: Length,
}

impl<'a, M> Item<'a, M> {
    pub fn on_press(mut self, msg: M) -> Self {
        self.on_press = Some(msg);
        self
    }

    pub fn on_toggle<F>(mut self, msg: F) -> Self
    where
        F: Fn(bool) -> M + 'a,
    {
        self.on_toggle = Some(Box::new(msg));
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
}

impl<M> From<super::Item> for Item<'_, M> {
    fn from(value: super::Item) -> Self {
        Self {
            name: value.name,
            price: value.price,
            category: value.category,
            kind: value.kind,
            on_press: None,
            on_toggle: None,
            width: Length::Fixed(RECEIPT_WIDTH),
        }
    }
}

#[derive(Clone)]
pub enum Event {
    Press,
    Toggle(bool),
}

impl<M> Component<M> for Item<'_, M>
where
    M: Clone,
{
    type Event = Event;
    type State = ();

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<M> {
        match event {
            Event::Press => self.on_press.clone(),
            Event::Toggle(b) => self.on_toggle.as_ref().map(|f| f(b)),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        Button::new(
            column![
                Text::new(self.name.to_string()),
                match self.kind {
                    ItemKind::Regular { num: 0 } | ItemKind::Special | ItemKind::InStock(_) =>
                        row![SMALL_TEXT::new(format!("{} kr", self.price))
                            .width(Length::Fill)
                            .horizontal_alignment(Horizontal::Left)],
                    ItemKind::Regular { num } => row![
                        SMALL_TEXT::new(format!("{}x{} kr", num, self.price)),
                        SMALL_TEXT::new(format!("{} kr", num * self.price))
                            .width(Length::Fill)
                            .horizontal_alignment(Horizontal::Right),
                    ],
                },
                if let ItemKind::InStock(stock) = self.kind {
                    Checkbox::new("I Lager", stock)
                        .text_size(SMALL_TEXT::size())
                        .on_toggle(Event::Toggle)
                        .into()
                } else {
                    Element::new(column![])
                }
            ]
            .height(Length::Shrink)
            .spacing(SMALL_PADDING),
        )
        .padding(DEF_PADDING)
        .width(self.width)
        .style(if self.on_press.is_some() {
            self.category.into()
        } else {
            Container::Border
        })
        .on_press(Event::Press)
        .into()
    }
}

impl<'a, M> From<Item<'a, M>> for Element<'a, M>
where
    M: Clone + 'a,
{
    fn from(value: Item<'a, M>) -> Self {
        iced::widget::component(value)
    }
}
