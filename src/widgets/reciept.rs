use {
    super::Clickable,
    crate::{
        payment::Payment,
        screens::transactions::Item,
        styles::{DEF_PADDING, RECIEPT_WIDTH},
    },
    iced::{
        button,
        scrollable::{self, Scrollable},
        Column, Element, Length, Text,
    },
    indexmap::IndexMap,
};

#[derive(Debug, Clone)]
pub struct Reciept<M> {
    scroll: scrollable::State,
    click: button::State,
    message: Option<M>,
    items: IndexMap<String, Item>,
    sum: i32,
    payment: Payment,
}

impl<M> Reciept<M>
where
    M: Clone,
{
    pub fn new() -> Self {
        Self::new_from(IndexMap::new(), 0, Payment::Swish)
    }

    pub fn new_from(items: IndexMap<String, Item>, sum: i32, payment: Payment) -> Self {
        Self {
            scroll: scrollable::State::new(),
            click: button::State::new(),
            message: None,
            items,
            sum,
            payment,
        }
    }

    pub fn add(&mut self, item: Item) {
        self.sum += item.price_total();
        match self.items.get_mut(&item.name()) {
            Some(it) => {
                *it = match it.clone() {
                    Item::Regular { name, price, num } => Item::Regular {
                        name,
                        price,
                        num: num + item.num(),
                    },
                    Item::Special { name, price } => Item::Special {
                        name,
                        price: price + item.num(),
                    },
                };
            }
            None => {
                self.items.insert(item.name(), item);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn sum(&self) -> i32 {
        self.sum
    }

    pub fn json(&self) -> String {
        serde_json::ser::to_string(&self.items).unwrap()
    }

    pub fn on_press(mut self, msg: M) -> Self {
        self.message = Some(msg);
        self
    }

    pub fn view(&mut self) -> Element<M> {
        let body = Clickable::new(
            &mut self.click,
            Column::new()
                .push(
                    self.items
                        .values_mut()
                        .fold(
                            Scrollable::new(&mut self.scroll).spacing(DEF_PADDING),
                            |col, item| col.push(item.view()),
                        )
                        .height(Length::Fill),
                )
                .push(Text::new(format!("Total: {}kr", self.sum)))
                .width(Length::Units(RECIEPT_WIDTH))
                .spacing(DEF_PADDING),
        )
        .padding(0)
        .height(Length::Fill);
        match &self.message {
            Some(msg) => body.on_press(msg.clone()),
            None => body,
        }
        .into()
    }
}
