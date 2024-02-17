use {category::*, iced::Element};

pub mod category;
pub mod component;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ItemKind {
    Special,
    Regular { num: i32 },
    InStock(bool),
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub price: i32,
    pub category: Category,
    kind: ItemKind,
}

impl Item {
    pub fn new_menu(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            name: row.get("name")?,
            price: row.get("price")?,
            category: row.get("category")?,
            kind: if row.get("special")? {
                ItemKind::Special
            } else {
                ItemKind::Regular { num: 0 }
            },
        })
    }

    pub fn new_sales(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            name: row.get("item")?,
            price: row.get("price")?,
            category: Category::Other, //Not relevant
            kind: if row.get("special")? {
                ItemKind::Special
            } else {
                ItemKind::Regular {
                    num: row.get("amount")?,
                }
            },
        })
    }

    pub fn new_stock(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            name: row.get("name")?,
            price: row.get("price")?,
            category: row.get("category")?,
            kind: ItemKind::InStock(row.get("available")?),
        })
    }

    pub fn is_special(&self) -> bool {
        matches!(self.kind, ItemKind::Special)
    }

    pub fn has_amount(&self) -> Option<i32> {
        match self.kind {
            ItemKind::Regular { num } => Some(num),
            _ => None,
        }
    }

    pub fn set_amount(&mut self, num: i32) {
        self.kind = match self.kind {
            ItemKind::Regular { .. } => ItemKind::Regular { num },
            _ => unreachable!("set_amount on incompatible kind {:?}", self.kind),
        }
    }

    pub fn price_total(&self) -> i32 {
        match self.kind {
            ItemKind::Regular { num } => num * self.price,
            ItemKind::Special | ItemKind::InStock(_) => self.price,
        }
    }

    pub fn is_in_stock(&self) -> bool {
        !matches!(self.kind, ItemKind::InStock(false))
    }

    pub fn in_stock(&mut self, a: bool) {
        if let ItemKind::InStock(ref mut b) = self.kind {
            *b = a;
        }
    }

    pub fn on_press<'a, M>(self, msg: M) -> component::Item<'a, M> {
        component::Item::from(self).on_press(msg)
    }

    pub fn on_toggle<'a, F, M>(self, msg: F) -> component::Item<'a, M>
    where
        F: Fn(bool) -> M + 'a,
    {
        component::Item::from(self).on_toggle(msg)
    }
}

impl std::cmp::PartialEq for Item {
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name && (self.price == rhs.price || self.is_special())
    }
}

impl std::cmp::Eq for Item {}

impl std::hash::Hash for Item {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.name.hash(state);
        if !self.is_special() {
            self.price.hash(state);
        }
    }
}

///Adds two of the same item into one with their combined amounts
impl std::ops::Add for Item {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self != rhs {
            unreachable!("Tried to add different items:\n'{:#?}'\n'{:#?}'", self, rhs);
        }

        Self {
            name: self.name.clone(),
            price: if self.is_special() {
                self.price + rhs.price
            } else {
                self.price
            },
            category: self.category,
            kind: match (self.kind, rhs.kind) {
                (ItemKind::Special, ItemKind::Special) => ItemKind::Special,
                (ItemKind::Regular { num }, ItemKind::Regular { num: num2 }) => {
                    ItemKind::Regular { num: num + num2 }
                }
                _ => unreachable!(),
            },
        }
    }
}

impl std::ops::AddAssign for Item
where
    Self: std::ops::Add<Output = Self>,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

impl<'a, M> From<Item> for Element<'a, M>
where
    M: Clone + 'a,
{
    fn from(value: Item) -> Self {
        iced::widget::component(component::Item::from(value))
    }
}
