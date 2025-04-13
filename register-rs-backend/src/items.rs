use crate::Result;
use chrono::{DateTime, Local};
use getset::{Getters, WithSetters};
use rusqlite::params;

pub mod category;

pub use category::Category;

#[derive(Default, Debug, Clone, PartialEq, Eq, Getters, WithSetters)]
#[getset(get = "pub", set_with = "pub")]
pub struct Item {
    name: String,
    price: i32,
    available: Option<bool>,
    #[getset(skip)]
    special: bool,
    category: Category,
}

impl Item {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_special(&self) -> bool {
        self.special
    }

    pub(crate) fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            name: row.get("name").or_else(|_| row.get("item"))?,
            price: row.get("price")?,
            available: row.get("available").ok(),
            special: row.get("special").unwrap_or(false),
            category: row.get("category").unwrap_or(Category::Other),
        })
    }

    pub async fn get_all() -> Result<Vec<Self>> {
        select!(
            "SELECT name, price, available, special, category FROM menu
                ORDER BY
                    special ASC,
                    CASE category
                        WHEN 'alcohol' THEN 1
                        WHEN 'drink' THEN 2
                        WHEN 'food' THEN 3
                        WHEN 'other' THEN 4
                        ELSE 5
                    END,
                    name DESC",
            Self::from_row
        )
    }

    pub async fn get_all_available() -> Result<Vec<Self>> {
        select!(
            "SELECT name, price, special, category FROM menu
                WHERE available=true
                ORDER BY
                    special ASC,
                    CASE category
                        WHEN 'alcohol' THEN 1
                        WHEN 'drink' THEN 2
                        WHEN 'food' THEN 3
                        WHEN 'other' THEN 4
                        ELSE 5
                    END,
                    name DESC",
            Self::from_row
        )
    }

    pub async fn insert_sale(self, time: DateTime<Local>, amount: i32) -> Result<()> {
        insert!(
            "INSERT INTO receipt_item (receipt, item, amount, price) VALUES (?1, ?2, ?3, ?4)",
            params![time, self.name, amount, self.price,]
        )?;
        Ok(())
    }

    pub async fn insert_new(self) -> Result<()> {
        insert!(
            "INSERT INTO menu (name, price, available, category) 
                VALUES (?1, ?2, true, ?3)",
            params![self.name, self.price, self.category]
        )?;
        Ok(())
    }

    pub async fn update(self, new: Self) -> Result<()> {
        insert!(
            "UPDATE menu SET name=?1, price=?2, category=?3 WHERE name=?4",
            params![new.name, new.price, new.category, self.name]
        )?;
        Ok(())
    }

    pub async fn change_availability(self, availability: bool) -> Result<()> {
        insert!(
            "UPDATE menu SET available=?1 WHERE name=?2",
            params![availability, self.name]
        )?;
        Ok(())
    }
}

impl std::hash::Hash for Item {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.name.hash(state);
        if !self.special {
            self.price.hash(state);
        }
    }
}
