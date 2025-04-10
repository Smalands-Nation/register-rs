use crate::Result;

pub mod category;
pub use category::Category;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    name: String,
    price: i32,
    available: bool,
    special: bool,
    category: Category,
}

impl Item {
    pub(crate) fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            name: row.get("name")?,
            price: row.get("price")?,
            available: row.get("available").unwrap_or(true),
            special: row.get("special").unwrap_or(false),
            category: row.get("category")?,
        })
    }

    pub async fn get_all() -> Result<Vec<Self>> {
        sql!(
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
        sql!(
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
