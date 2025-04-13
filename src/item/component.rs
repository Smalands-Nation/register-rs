use {
    crate::{
        theme::{Container, DEF_PADDING, RECEIPT_WIDTH, SMALL_PADDING},
        widgets::{column, row, SMALL_TEXT},
    },
    backend::items::{Category, Item as RawItem},
    iced::{
        alignment::Horizontal,
        widget::{Button, Checkbox, Component, Text},
        Color, Element, Length,
    },
};

pub struct Item<'a, M> {
    item: RawItem,
    amount: Option<i32>,
    on_press: Option<M>,
    on_toggle: Option<Box<dyn Fn(bool) -> M + 'a>>,
    width: Length,
}

impl<'a, M> Item<'a, M> {
    pub fn new(item: RawItem, amount: i32) -> Self {
        Self {
            item,
            amount: Some(amount),
            on_press: None,
            on_toggle: None,
            width: Length::Fixed(RECEIPT_WIDTH),
        }
    }

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

impl<M> From<RawItem> for Item<'_, M> {
    fn from(item: RawItem) -> Self {
        Self {
            item,
            amount: None,
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
                Text::new(self.item.name()),
                match self.amount {
                    None | Some(0) => row![SMALL_TEXT::new(format!("{} kr", self.item.price()))
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Left)],
                    Some(num) if self.item.is_special() =>
                        row![SMALL_TEXT::new(format!("{} kr", num * self.item.price()))
                            .width(Length::Fill)
                            .horizontal_alignment(Horizontal::Right),],
                    Some(num) => row![
                        SMALL_TEXT::new(format!("{}x{} kr", num, self.item.price())),
                        SMALL_TEXT::new(format!("{} kr", num * self.item.price()))
                            .width(Length::Fill)
                            .horizontal_alignment(Horizontal::Right),
                    ],
                },
                if let Some(stock) = self.item.available() {
                    Checkbox::new("I Lager", *stock)
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
            Container::BorderFill(match self.item.category() {
                Category::Alcohol => Color::from_rgb8(0xFF, 0x6F, 0x59),
                Category::Drink => Color::from_rgb8(0xC0, 0xDA, 0x74),
                Category::Food => Color::from_rgb8(0xA7, 0xC6, 0xDA),
                Category::Other => Color::WHITE,
            })
        } else {
            Container::Border
        })
        .clip(true)
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
