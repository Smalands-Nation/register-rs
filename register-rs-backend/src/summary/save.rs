use {
    crate::{
        items::Item,
        receipts::{Payment, Receipt, print::FONT},
        summary::Summary,
    },
    chrono::{DateTime, Local},
    genpdf::{
        Alignment, Document, Element, SimplePageDecorator,
        elements::{Break, Image, LinearLayout, Paragraph, TableLayout, Text},
        fonts,
        style::Style,
    },
    indexmap::IndexSet,
    std::{collections::HashMap, io::Cursor, path::PathBuf, sync::Arc},
};

struct Stats {
    //Used for consistent ordering
    payments: Vec<Payment>,
    items: IndexSet<Item>,
    item_counts: HashMap<(Payment, Item), i32>,
}

impl Stats {
    fn new(data: &HashMap<Payment, Receipt>) -> Self {
        let payments = data.keys().copied().collect();

        let (mut items, item_counts) = data
            .iter()
            .flat_map(|(payment, receipt)| {
                receipt
                    .iter()
                    .map(|(item, amount)| (*payment, item, amount))
            })
            .fold(
                (IndexSet::new(), HashMap::new()),
                |(mut hs, mut hm), (payment, item, amount)| {
                    hs.insert(item.clone());
                    *hm.entry((payment, item.clone())).or_insert(0) += amount;
                    (hs, hm)
                },
            );

        //Make sure special items are last, order of normal items does not matter
        items.sort_by(|v1, v2| match (v1.is_special(), v2.is_special()) {
            (false, false) | (true, true) => std::cmp::Ordering::Equal,
            (false, true) => std::cmp::Ordering::Less,
            (true, false) => std::cmp::Ordering::Greater,
        });

        Self {
            payments,
            items,
            item_counts,
        }
    }

    fn create_pdf(
        self,
        path: impl Into<PathBuf>,
        (from, to): (DateTime<Local>, DateTime<Local>),
    ) -> Result<PathBuf> {
        let mut doc = Document::new(fonts::FontFamily {
            regular: FONT.clone(),
            bold: FONT.clone(),
            italic: FONT.clone(),
            bold_italic: FONT.clone(),
        });
        doc.set_paper_size((297, 210));

        doc.set_page_decorator({
            let mut dec = SimplePageDecorator::new();
            dec.set_margins(10);
            dec.set_header(|_| {
                let mut header = TableLayout::new(vec![1, 1]);
                header
                    .row()
                    .element({
                        let logga = Cursor::new(include_bytes!("../../../resources/logga.png"));
                        Image::from_reader(logga)
                            .unwrap()
                            .with_alignment(Alignment::Left)
                            .with_scale((0.5, 0.5))
                    })
                    .element({
                        ["Smålands Nation", "Nyhemsgatan 30", "302 49 Halmstad"]
                            .into_iter()
                            .map(|s| Paragraph::new(s).aligned(Alignment::Right))
                            .fold(LinearLayout::vertical(), |list, text| list.element(text))
                    })
                    .push()
                    .expect("Couldn't table header");
                header
            });
            dec
        });

        doc.push(Break::new(1));

        doc.push(
            Text::new(format!(
                "Försäljning {}",
                if from == to {
                    from.format("%F").to_string()
                } else {
                    format!("för perioden {} tom {}", from.format("%F"), to.format("%F"))
                }
            ))
            .styled(Style::new().with_font_size(24)),
        );

        doc.push(Break::new(2));

        let mut table = TableLayout::new(vec![1; self.payments.len() + 3]);
        ["", "à-pris"]
            .into_iter()
            .map(String::from)
            .chain(self.payments.iter().map(ToString::to_string))
            .chain([String::from("Tot.")])
            .fold(table.row(), |row, e| {
                row.element(Text::new(e).padded(3).framed())
            })
            .push()
            .expect("Table header failed");

        let mut summary_tot = 0;
        let mut tot_by_payment = HashMap::new();
        for item in self.items.iter() {
            let mut row = table.row();

            row.push_element(Text::new(item.name()).padded(3).framed());
            row.push_element(
                Paragraph::new(if item.is_special() {
                    String::new()
                } else {
                    format!("{}kr", item.price())
                })
                .aligned(Alignment::Right)
                .padded(3)
                .framed(),
            );

            let mut item_tot = 0;
            for p in self.payments.iter() {
                row.push_element(
                    match self.item_counts.get(&(*p, item.clone())) {
                        Some(amount) => {
                            let price = item.price() * amount;
                            summary_tot += price;
                            item_tot += price;
                            *tot_by_payment.entry(p).or_insert(0) += price;

                            Paragraph::new(if item.is_special() {
                                format!("{}kr", price)
                            } else {
                                format!("{amount}st")
                            })
                        }
                        None => Paragraph::new(if item.is_special() { "0kr" } else { "0st" }),
                    }
                    .aligned(Alignment::Right)
                    .padded(3)
                    .framed(),
                )
            }

            row.element(
                Paragraph::new(format!("{item_tot}kr"))
                    .aligned(Alignment::Right)
                    .padded(3)
                    .framed(),
            )
            .push()
            .unwrap()
        }

        ["Tot.", ""]
            .into_iter()
            .map(String::from)
            .chain(self.payments.iter().map(|p| {
                tot_by_payment
                    .get(p)
                    .map(|tot| format!("{tot}kr"))
                    .unwrap_or_default()
            }))
            .chain([format!("{summary_tot}kr")])
            .fold(table.row(), |row, cell| {
                row.element(Paragraph::new(cell).padded(3).framed())
            })
            .push()
            .expect("Table footer failed");

        doc.push(table.padded(10));

        let mut path = path.into();
        path.push(if from == to {
            format!("{}.pdf", from.format("%F"))
        } else {
            format!("{}_{}.pdf", from.format("%F"), to.format("%F"))
        });
        doc.render_to_file(path.clone())
            .map_err(|e| Error::Pdf(Arc::new(e)))?;

        Ok(path)
    }
}

#[cfg(not(debug_assertions))]
pub async fn save(Summary { from, to, data }: &Summary) -> Result<PathBuf> {
    use chrono::Datelike;

    let mut path = dirs::document_dir().ok_or(Error::NoPath)?;
    path.push("sales");
    path.push(to.year().to_string());
    if let Err(e) = std::fs::create_dir_all(&path) {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            ek => return Err(Error::Io(ek)),
        }
    }

    Stats::new(data).create_pdf(path, (*from, *to))
}

#[cfg(debug_assertions)]
pub async fn save(Summary { from, to, data }: &Summary) -> Result<PathBuf> {
    Stats::new(data).create_pdf(".", (*from, *to))
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    NoPath,
    Io(std::io::ErrorKind),
    Pdf(Arc<genpdf::error::Error>),
}
