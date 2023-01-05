use {
    crate::{
        styles::{Bordered, DEF_PADDING, SMALL_PADDING},
        widgets::{row, SMALL_TEXT},
    },
    frost::clickable::Clickable,
    iced::{
        alignment::Horizontal,
        widget::{Checkbox, Column, Text},
        Color, Element, Length,
    },
    kind::*,
    rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput, ValueRef},
};

pub mod kind {
    pub trait ItemKind: Clone {}

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Sales {
        Special,
        Regular { num: i32 },
    }
    impl ItemKind for Sales {}

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Stock {
        pub idx: usize,
        pub available: bool,
    }
    impl ItemKind for Stock {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Alcohol,
    Drink,
    Food,
    Other,
}

impl Category {
    pub const ALL: [Self; 4] = [Self::Alcohol, Self::Drink, Self::Food, Self::Other];
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Alcohol => "Alkohol",
                Self::Drink => "Dryck",
                Self::Food => "Mat",
                Self::Other => "Övrigt",
            }
        )
    }
}

impl FromSql for Category {
    fn column_result(value: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            ValueRef::Text(b"alcohol") => Ok(Self::Alcohol),
            ValueRef::Text(b"drink") => Ok(Self::Drink),
            ValueRef::Text(b"food") => Ok(Self::Food),
            ValueRef::Text(b"other") => Ok(Self::Other),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Category {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(match self {
            Self::Alcohol => b"alcohol",
            Self::Drink => b"drink",
            Self::Food => b"food",
            Self::Other => b"other",
        })))
    }
}

impl From<Category> for Bordered {
    fn from(c: Category) -> Self {
        Bordered::new(match c {
            Category::Alcohol => Color::from_rgb8(0xFF, 0x6F, 0x59),
            Category::Drink => Color::from_rgb8(0xC0, 0xDA, 0x74),
            Category::Food => Color::from_rgb8(0xA7, 0xC6, 0xDA),
            Category::Other => Color::WHITE,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Item<K: ItemKind> {
    pub name: String,
    pub price: i32,
    pub category: Category,
    pub kind: K,
}

impl Item<Sales> {
    pub fn new_menu(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            name: row.get("name")?,
            price: row.get("price")?,
            category: row.get("category")?,
            kind: if row.get("special")? {
                Sales::Special
            } else {
                Sales::Regular { num: 0 }
            },
        })
    }

    pub fn new_sales(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            name: row.get("name")?,
            price: row.get("price")?,
            category: Category::Other, //Not relevant
            kind: if row.get("special")? {
                Sales::Special
            } else {
                Sales::Regular {
                    num: row.get("amount")?,
                }
            },
        })
    }

    pub fn is_special(&self) -> bool {
        matches!(self.kind, Sales::Special)
    }

    pub fn has_amount(&self) -> Option<i32> {
        match self.kind {
            Sales::Regular { num } => Some(num),
            _ => None,
        }
    }

    pub fn set_amount(&mut self, num: i32) {
        self.kind = match self.kind {
            Sales::Regular { .. } => Sales::Regular { num },
            _ => unreachable!("set_amount on Sales::Special"),
        }
    }

    pub fn price_total(&self) -> i32 {
        match self.kind {
            Sales::Regular { num } => num * self.price,
            Sales::Special => self.price,
        }
    }
}

impl Item<Stock> {
    pub fn new_stock(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            name: row.get("name")?,
            price: row.get("price")?,
            category: row.get("category")?,
            kind: Stock {
                idx: 0,
                available: row.get("available")?,
            },
        })
    }

    pub fn with_index(mut self, idx: usize) -> Self {
        self.kind.idx = idx;
        self
    }

    pub fn is_in_stock(&self) -> bool {
        self.kind.available
    }

    pub fn in_stock(&mut self, a: bool) {
        self.kind.available = a;
    }
}

impl<K> Item<K>
where
    K: ItemKind,
{
    pub fn as_widget<M: Clone>(&self) -> ItemWidget<K, M> {
        let Self {
            name,
            price,
            category,
            kind,
        } = self.clone();

        ItemWidget {
            name,
            price,
            category,
            kind,
            msg: None,
        }
    }

    pub fn on_press<F, M>(&self, msg: F) -> ItemWidget<K, M>
    where
        F: FnOnce(Self) -> M,
        M: Clone,
    {
        let mut comp = self.as_widget();
        //TODO maybe refactor messages of items only used for ID's to reduce clones
        comp.msg = Some(msg(self.clone()));
        comp
    }
}

impl std::cmp::PartialEq for Item<Sales> {
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name && (self.price == rhs.price || self.is_special())
    }
}

impl std::cmp::Eq for Item<Sales> {}

impl std::hash::Hash for Item<Sales> {
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
impl std::ops::Add for Item<Sales> {
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
                (Sales::Special, Sales::Special) => Sales::Special,
                (Sales::Regular { num }, Sales::Regular { num: num2 }) => {
                    Sales::Regular { num: num + num2 }
                }
                _ => unreachable!(),
            },
        }
    }
}

impl std::ops::AddAssign for Item<Sales> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

pub struct ItemWidget<K, M>
where
    K: ItemKind,
    M: Clone,
{
    name: String,
    price: i32,
    category: Category,
    kind: K,
    msg: Option<M>,
    //color: bool,
}

//NOTE use macro due to wierd scoping
macro_rules! item_widget {
    ($($child:expr),*) => {
        Clickable::new(
            Column::new()
            .spacing(SMALL_PADDING)
            $(.push($child))*
            ,
        )
        .padding(DEF_PADDING)
        .width(Length::Fill)
        //TODO fix styles later
        //.style(if self.color {
        //    self.inner.category.into()
        //} else {
        //    Bordered::default()
        //});

    }
}

impl<'a, M> From<ItemWidget<Sales, M>> for Element<'a, M>
where
    M: Clone + 'a,
{
    fn from(i: ItemWidget<Sales, M>) -> Self {
        let w = item_widget![
            Text::new(i.name.to_string()),
            match i.kind {
                Sales::Regular { num: 0 } | Sales::Special => row![
                    #nopad
                    SMALL_TEXT::new(format!("{} kr", i.price))
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Left),
                ],
                Sales::Regular { num } => row![
                    #nopad
                    SMALL_TEXT::new(format!("{}x{} kr", num, i.price)),
                    SMALL_TEXT::new(format!("{} kr", num* i.price))
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Right),
                ],
            }
        ];

        match i.msg {
            Some(msg) => w.on_press(msg),
            None => w,
        }
        .into()
    }
}

use crate::screens::manager::Message;
impl From<ItemWidget<Stock, Message>> for Element<'_, Message> {
    fn from(i: ItemWidget<Stock, Message>) -> Self {
        let w = item_widget![
            Text::new(i.name.to_string()),
            row![#nopad
                SMALL_TEXT::new(format!("{} kr", i.price))
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Left),
            ],
            Checkbox::new(i.kind.available, "I Lager", move |b| {
                Message::ToggleItem(i.kind.idx, b)
            })
        ];

        match i.msg {
            Some(msg) => w.on_press(msg),
            None => w,
        }
        .into()
    }
}
