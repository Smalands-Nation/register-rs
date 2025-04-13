use crate::{
    Result,
    receipts::{Payment, RawEntry, Receipt},
};
use chrono::{DateTime, Local};
use getset::Getters;
use rusqlite::params;
use std::{collections::HashMap, path::PathBuf};
use strum::VariantArray;

pub(crate) mod save;

#[derive(Debug, Default, Clone, Getters)]
#[getset(get = "pub")]
pub struct Summary {
    from: DateTime<Local>,
    to: DateTime<Local>,
    #[getset(skip)]
    data: HashMap<Payment, Receipt>,
}

impl Summary {
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn receipts(&self) -> impl Iterator<Item = (&Payment, &Receipt)> {
        self.data.iter()
    }

    pub async fn get_sales_summary(from: DateTime<Local>, to: DateTime<Local>) -> Result<Self> {
        let data = select!(
            "SELECT item, amount, price, special, method FROM receipts_view \
                WHERE time BETWEEN ?1 AND ?2",
            params![from, to],
            RawEntry::from_row,
            ..
        )
        .fold(
            Result::Ok(HashMap::with_capacity(Payment::VARIANTS.len())),
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
                        .or_insert_with(|| Receipt::new(time, payment));
                    r.insert(item, amount);
                    hm
                })
            },
        )?;

        Ok(Self { from, to, data })
    }

    pub async fn save(&self) -> Result<PathBuf> {
        Ok(save::save(self).await?)
    }
}
