use {
    crate::{
        item::{kind::Sales, Item},
        payment::Payment,
        theme::{DEF_PADDING, RECEIPT_WIDTH},
        widgets::column,
        Element, Renderer,
    },
    frost::clickable::Clickable,
    iced::{
        widget::{Column, Scrollable, Text},
        Length,
    },
    iced_lazy::Component,
    indexmap::IndexSet,
};

#[derive(Debug, Clone)]
pub struct Receipt<M> {
    pub items: IndexSet<Item<Sales>>,
    pub sum: i32,
    pub payment: Payment,
    msg: Option<M>,
}

impl<M> Receipt<M> {
    pub fn new(payment: Payment) -> Self {
        Self::new_from(IndexSet::new(), 0, payment)
    }

    pub fn new_from(items: IndexSet<Item<Sales>>, sum: i32, payment: Payment) -> Self {
        Self {
            items,
            sum,
            payment,
            msg: None,
        }
    }

    pub fn add(&mut self, item: Item<Sales>) {
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
            .sort_by(|v1, v2| match (v1.is_special(), v2.is_special()) {
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

    //pub fn as_widget<M>(&self) -> ReceiptWidget<M> {
    //    ReceiptWidget {
    //        message: None,
    //        inner: self,
    //    }
    //}
}

//#[derive(Debug)]
//pub struct ReceiptWidget<'a, M> {
//    message: Option<M>,
//    inner: &'a Receipt,
//}

//impl<'a, M> ReceiptWidget<'a, M>
//where
//    M: Clone + 'a,
//{
//    pub fn on_press(mut self, msg: M) -> Self {
//        self.message = Some(msg);
//        self
//    }
//}

impl<'a, M> Component<M, Renderer> for Receipt<M>
where
    M: Clone + 'a,
{
    type Event = bool;
    type State = ();

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<M> {
        if event {
            self.msg.clone()
        } else {
            None
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        Clickable::new(
            column![
                #nopad
                Scrollable::new(
                    Column::with_children(
                        self
                            .items
                            .iter()
                            .map(|item| item.as_widget(false).into())
                            .collect(),
                    )
                    .spacing(DEF_PADDING),
                )
                .scrollbar_width(10)
                .height(Length::Fill),
                Text::new(format!("Total: {}kr", self.sum)),
            ]
            .width(Length::Units(RECEIPT_WIDTH))
            .spacing(DEF_PADDING),
        )
        .padding(0)
        .height(Length::Fill)
        .on_press(true)
        .into()
    }
}

impl<'a, M> From<Receipt<M>> for Element<'a, M>
where
    M: Clone + 'a,
{
    fn from(value: Receipt<M>) -> Self {
        iced_lazy::component(value)
    }
}
