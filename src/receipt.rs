use {
    crate::{
        item::Item,
        payment::Payment,
        styles::{DEF_PADDING, RECEIPT_WIDTH},
        widgets::Clickable,
    },
    iced::{
        pure::{
            widget::{Column, Scrollable, Text},
            Element,
        },
        Length,
    },
    indexmap::IndexSet,
};

#[derive(Debug, Clone)]
pub struct Receipt {
    pub items: IndexSet<Item>,
    pub sum: i32,
    pub payment: Payment,
}

impl Receipt {
    pub fn new(payment: Payment) -> Self {
        Self::new_from(IndexSet::new(), 0, payment)
    }

    pub fn new_from(items: IndexSet<Item>, sum: i32, payment: Payment) -> Self {
        Self {
            items,
            sum,
            payment,
        }
    }

    pub fn add(&mut self, item: Item) {
        self.sum += item.price_total();
        let it = self.items.get(&item).cloned();
        match it {
            Some(it) => {
                self.items.replace(it + item);
            }
            None => {
                self.items.insert(item);
            }
        }
        self.items
            .sort_by(|v1, v2| match (v1.special(), v2.special()) {
                (false, false) | (true, true) => std::cmp::Ordering::Equal,
                (false, true) => std::cmp::Ordering::Less,
                (true, false) => std::cmp::Ordering::Greater,
            });
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn as_widget<M>(&mut self) -> ReceiptWidget<M> {
        ReceiptWidget {
            message: None,
            inner: self,
        }
    }
}

#[derive(Debug)]
pub struct ReceiptWidget<'a, M> {
    message: Option<M>,
    inner: &'a mut Receipt,
}

impl<'a, M> ReceiptWidget<'a, M>
where
    M: Clone + 'a,
{
    pub fn on_press(mut self, msg: M) -> Self {
        self.message = Some(msg);
        self
    }
}

impl<'a, M> From<ReceiptWidget<'a, M>> for Element<'a, M>
where
    M: Clone + 'a,
{
    fn from(r: ReceiptWidget<'a, M>) -> Self {
        let body = Clickable::new(
            Column::new()
                .push(
                    Scrollable::new(
                        r.inner
                            .items
                            .iter()
                            .fold(Column::new().spacing(DEF_PADDING), |col, item| {
                                col.push(item.as_widget())
                            }),
                    )
                    .scrollbar_width(10)
                    .height(Length::Fill),
                )
                .push(Text::new(format!("Total: {}kr", r.inner.sum)))
                .width(Length::Units(RECEIPT_WIDTH))
                .spacing(DEF_PADDING),
        )
        .padding(0)
        .height(Length::Fill);
        match &r.message {
            Some(msg) => body.on_press(msg.clone()),
            None => body,
        }
        .into()
    }
}
