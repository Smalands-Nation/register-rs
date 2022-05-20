use {
    crate::{error::Result, item::Item, payment::Payment, receipt::Receipt},
    chrono::{Date, Local},
    genpdf::{
        elements::{Break, Image, LinearLayout, Paragraph, TableLayout, Text},
        fonts,
        style::Style,
        Alignment, Document, Element, SimplePageDecorator,
    },
    indexmap::{IndexMap, IndexSet},
    std::{io::Cursor, path::PathBuf},
};

fn create_pdf(
    path: impl Into<PathBuf>,
    stats: IndexSet<(Payment, Item)>,
    (from, to): (Date<Local>, Date<Local>),
) -> Result<PathBuf> {
    let font = fonts::FontData::new(
        if let iced::Font::External { bytes, .. } = crate::FONT {
            bytes.to_vec()
        } else {
            vec![]
        },
        None,
    )
    .unwrap();

    let mut doc = Document::new(fonts::FontFamily {
        regular: font.clone(),
        bold: font.clone(),
        italic: font.clone(),
        bold_italic: font,
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

    let (payments, mut items): (IndexSet<_>, IndexSet<_>) = stats.clone().into_iter().unzip();
    items.sort_by(|v1, v2| match (v1.special(), v2.special()) {
        (false, false) | (true, true) => {
            if v1.name == v2.name {
                v1.price.cmp(&v2.price)
            } else {
                v1.name.cmp(&v2.name)
            }
        }
        (false, true) => std::cmp::Ordering::Less,
        (true, false) => std::cmp::Ordering::Greater,
    });

    let mut table = TableLayout::new(vec![1; payments.len() + 3]);
    ["", "à-pris"]
        .into_iter()
        .map(String::from)
        .chain(payments.iter().map(|v| String::from(*v)))
        .chain([String::from("Tot.")])
        .fold(table.row(), |row, e| {
            row.element(Text::new(e).padded(3).framed())
        })
        .push()
        .expect("Table header failed");

    let mut tots = IndexMap::new();
    for item in items.iter() {
        let mut row = table.row();

        row.push_element(Text::new(item.name.clone()).padded(3).framed());
        row.push_element(
            Paragraph::new(if item.special() {
                String::new()
            } else {
                format!("{}kr", item.price)
            })
            .aligned(Alignment::Right)
            .padded(3)
            .framed(),
        );

        let mut tot = 0;
        for p in payments.iter() {
            row.push_element(
                match stats.get(&(*p, item.clone())) {
                    Some((_, item)) => {
                        let price = item.price_total();
                        match tots.get_mut(p) {
                            Some(t) => {
                                *t += price;
                            }
                            None => {
                                tots.insert(p, price);
                            }
                        }
                        tot += price;

                        Paragraph::new(format!(
                            "{}st",
                            match item.num {
                                Some(n) => n,
                                None => item.price_total(),
                            }
                        ))
                    }
                    None => Paragraph::new("0st"),
                }
                .aligned(Alignment::Right)
                .padded(3)
                .framed(),
            )
        }

        row.element(
            Paragraph::new(format!("{}kr", tot))
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
        .chain(tots.values().map(|v| v.to_string()))
        .chain([tots.values().sum::<i32>().to_string()])
        .enumerate()
        .fold(table.row(), |row, (i, e)| {
            row.element(
                if i <= 1 {
                    Paragraph::new(e)
                } else {
                    Paragraph::new(e + "kr").aligned(Alignment::Right)
                }
                .padded(3)
                .framed(),
            )
        })
        .push()
        .expect("Table footer failed");

    doc.push(table.padded(10));

    let mut path = path.into();
    path.push(if from == to {
        format!("{}.pdf", from.format("%F"))
    } else {
        format!("{} {}", from.format("%F"), to.format("%F"))
    });
    doc.render_to_file(path.clone())
        .expect("Failed to write PDF file");

    Ok(path)
}

fn make_stats(data: IndexMap<Payment, Receipt>) -> IndexSet<(Payment, Item)> {
    data.into_values().fold(IndexSet::new(), |hm, r| {
        r.items.into_iter().fold(hm, |mut hm, item| {
            hm.insert((r.payment, item));
            hm
        })
    })
}

#[cfg(not(debug_assertions))]
use chrono::Datelike;
#[cfg(not(debug_assertions))]
pub async fn save(
    data: IndexMap<Payment, Receipt>,
    (from, to): (Date<Local>, Date<Local>),
) -> Result<PathBuf> {
    let mut path = dirs::document_dir().ok_or("No document path")?;
    path.push("sales");
    path.push(to.year().to_string());
    match std::fs::create_dir_all(&path) {
        Ok(_) => (),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            ek => return Err(ek.into()),
        },
    }

    let stats = make_stats(data);
    create_pdf(path, stats, (from, to))
}

#[cfg(debug_assertions)]
pub async fn save(
    data: IndexMap<Payment, Receipt>,
    (from, to): (Date<Local>, Date<Local>),
) -> Result<PathBuf> {
    let stats = make_stats(data);
    create_pdf(".", stats, (from, to))
}
