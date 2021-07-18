pub use crate::screens::transactions::Item;
use {crate::payment::Payment, indexmap::IndexMap};

#[derive(Debug, Clone)]
pub struct Reciept {
    pub items: IndexMap<String, Item>,
    pub sum: i32,
    pub payment: Payment,
}

impl Reciept {
    pub fn new() -> Self {
        Self::new_from(IndexMap::new(), 0, Payment::Swish)
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
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn json(&self) -> String {
        serde_json::ser::to_string(&self.items).unwrap()
    }
}
