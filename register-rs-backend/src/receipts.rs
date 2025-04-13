use crate::{Result, items::Item};
use chrono::{DateTime, Local};
use getset::WithSetters;
use indexmap::IndexMap;
use rusqlite::{Row, params};
use std::collections::HashMap;
use strum::VariantArray;

pub mod payments;
pub(crate) mod print;

pub use payments::Payment;

#[derive(Debug, Default, Clone, WithSetters)]
pub struct Receipt {
    //Item -> Amount
    items: IndexMap<Item, i32>,
    #[getset(set_with = "pub")]
    time: DateTime<Local>,
    #[getset(set_with = "pub")]
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

    pub fn insert(&mut self, item: Item, amount: i32) {
        *self.items.entry(item).or_insert(0) += amount;
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn sum(&self) -> i32 {
        self.items
            .iter()
            .map(|(item, amount)| item.price() * amount)
            .sum::<i32>()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Item, &i32)> {
        self.items.iter()
    }

    pub async fn get_sales_summary(
        from_time: DateTime<Local>,
        to_time: DateTime<Local>,
    ) -> Result<HashMap<Payment, Self>> {
        select!(
            "SELECT item, amount, price, special, method FROM receipts_view \
                WHERE time BETWEEN ?1 AND ?2",
            params![from_time, to_time],
            RawEntry::from_row,
            ..
        )
        .fold(
            Ok(HashMap::with_capacity(Payment::VARIANTS.len())),
            |res, raw| {
                let RawEntry {
                    time,
                    amount,
                    item,
                    payment,
                } = raw?;
                res.map(|mut hm| {
                    let r = hm
                        .entry(payment)
                        .or_insert_with(|| Self::new(time, payment));
                    r.insert(item, amount);
                    hm
                })
            },
        )
    }

    pub async fn get_recents() -> Result<IndexMap<DateTime<Local>, Self>> {
        select!(
            "SELECT * FROM receipts_view \
                WHERE time > date('now','-1 day') ORDER BY time DESC",
            RawEntry::from_row,
            ..
        )
        .fold(Ok(IndexMap::new()), |res, raw| {
            let RawEntry {
                time,
                amount,
                item,
                payment,
            } = raw?;
            res.map(|mut hm| {
                let r = hm.entry(time).or_insert_with(|| Self::new(time, payment));
                r.insert(item, amount);
                hm
            })
        })
    }

    pub async fn print(&self) -> Result<()> {
        Ok(print::print(self).await?)
    }

    pub async fn insert_sale(mut self) -> Result<()> {
        //FIXME start transaction? otherwise could get incomplete receipts
        insert!(
            "INSERT INTO receipts (time, method) VALUES (?1, ?2)",
            params![self.time, self.payment]
        )?;

        //FIXME (not tested) this might inf-lock the db since we dont drop the handle from above
        for (item, amount) in self.items.drain(..) {
            item.insert_sale(self.time, amount).await?;
        }

        Ok(())
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
