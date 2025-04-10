use crate::{Result, items::Item};
use chrono::{DateTime, Local};
use indexmap::{IndexMap, IndexSet};
use rusqlite::Row;
use std::collections::HashMap;
use strum::VariantArray;

pub mod payments;

pub use payments::Payment;

#[derive(Default)]
pub struct Receipt {
    items: IndexSet<(i32, Item)>,
    time: DateTime<Local>,
    sum: i32,
    payment: Payment,
}

impl Receipt {
    fn new(time: DateTime<Local>, payment: Payment) -> Self {
        Self {
            time,
            payment,
            ..Default::default()
        }
    }

    fn insert(&mut self, amount: i32, item: Item) {
        self.items.insert((amount, item));
    }

    pub async fn get_sales_summary(
        from_time: DateTime<Local>,
        to_time: DateTime<Local>,
    ) -> Result<HashMap<Payment, Self>> {
        let raw: Vec<RawEntry> = sql!(
            "SELECT item, amount, price, special, method FROM receipts_view \
            WHERE time BETWEEN ?1 AND ?2",
            params![from_time, to_time],
            RawEntry::from_row
        )?;

        Ok(raw.into_iter().fold(
            HashMap::with_capacity(Payment::VARIANTS.len()),
            |mut map, raw| {
                let RawEntry {
                    time,
                    amount,
                    item,
                    payment,
                } = raw;
                let r = map
                    .entry(payment)
                    .or_insert_with(|| Self::new(time, payment));
                r.insert(amount, item);
                map
            },
        ))
    }

    pub async fn get_recents() -> Result<IndexMap<DateTime<Local>, Self>> {
        let raw: Vec<RawEntry> = sql!(
            "SELECT * FROM receipts_view \
                    WHERE time > date('now','-1 day') ORDER BY time DESC",
            RawEntry::from_row
        )?;

        Ok(raw.into_iter().fold(IndexMap::new(), |mut map, raw| {
            let RawEntry {
                time,
                amount,
                item,
                payment,
            } = raw;
            let r = map.entry(time).or_insert_with(|| Self::new(time, payment));
            r.insert(amount, item);
            map
        }))
    }
}

struct RawEntry {
    time: DateTime<Local>,
    amount: i32,
    item: Item,
    payment: Payment,
}

impl RawEntry {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            time: row.get("time").unwrap_or_default(),
            amount: row.get("amount")?,
            item: Item::from_row(row)?,
            payment: row.get("payment").unwrap_or_default(),
        })
    }
}
