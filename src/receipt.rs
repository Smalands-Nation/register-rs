pub use crate::screens::transactions::Item;
use {crate::payment::Payment, indexmap::IndexMap};

#[derive(Debug, Clone)]
pub struct Receipt {
    pub items: IndexMap<String, Item>,
    pub sum: i32,
    pub payment: Payment,
}

impl Receipt {
    pub fn new(payment: Payment) -> Self {
        Self::new_from(IndexMap::new(), 0, payment)
    }

    pub fn new_from(items: IndexMap<String, Item>, sum: i32, payment: Payment) -> Self {
        Self {
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
        self.items.sort_by(|_, v1, _, v2| match (v1, v2) {
            (Item::Regular { .. }, Item::Regular { .. })
            | (Item::Special { .. }, Item::Special { .. }) => std::cmp::Ordering::Equal,
            (Item::Regular { .. }, Item::Special { .. }) => std::cmp::Ordering::Less,
            (Item::Special { .. }, Item::Regular { .. }) => std::cmp::Ordering::Greater,
        });
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
