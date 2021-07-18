use {
    crate::{
        error::Result,
        reciept::{Item, Reciept},
        styles::DEF_TEXT,
    },
    chrono::{DateTime, Local},
    genpdf::{
        elements::{Break, Image, Paragraph, TableLayout, Text},
        fonts, Alignment, Document, Mm, SimplePageDecorator,
    },
    std::io::Cursor,
};

/*
const fn pt_to_mm(pt: f64) -> Mm {
    (pt * 0.352_778_f64).into()
}
*/

const PAGE_WIDTH: f64 = 203.200_128; //pt_to_mm(8.0 * 72.0);
const PAGE_HEIGHT: f64 = 279.400_176; //pt_to_mm(11.0 * 72.0);

pub async fn print(reciept: Reciept, time: DateTime<Local>) -> Result<()> {
    let font = fonts::FontData::new(
        if let iced::Font::External { bytes, .. } = crate::FONT {
            bytes.to_vec()
        } else {
            vec![]
        },
        None,
    )
    .unwrap();

    let mut doc = genpdf::Document::new(fonts::FontFamily {
        regular: font.clone(),
        bold: font.clone(),
        italic: font.clone(),
        bold_italic: font,
    });
    doc.set_font_size(DEF_TEXT as u8);
    doc.set_paper_size((PAGE_WIDTH, PAGE_HEIGHT));
    doc.set_page_decorator({
        let mut dec = SimplePageDecorator::new();
        dec.set_margins((10, 5));
        dec
    });

    let logga = Cursor::new(include_bytes!("../resources/logga.png"));
    doc.push(
        Image::from_reader(logga)
            .unwrap()
            .with_alignment(Alignment::Center),
    );

    doc.push(Paragraph::new("Smålands Nation").aligned(Alignment::Center));
    doc.push(Paragraph::new("Nyhemsgatan 30").aligned(Alignment::Center));
    doc.push(Paragraph::new("302 49 Halmstad").aligned(Alignment::Center));
    doc.push(Paragraph::new("–".repeat(24)).aligned(Alignment::Center));

    for item in reciept.items.values() {
        match item {
            Item::Regular { name, price, num } => {
                doc.push(Text::new(name));
                doc.push({
                    let mut tbl = TableLayout::new(vec![1, 1]);
                    tbl.row()
                        .element(Text::new(format!(" {}x{}kr", num, price)))
                        .element(
                            Paragraph::new(format!("{}kr", item.price_total()))
                                .aligned(Alignment::Right),
                        )
                        .push()
                        .expect("Couldn't Table Price");
                    tbl
                });
            }
            Item::Special { name, price } => {
                doc.push(Text::new(name));
                doc.push(Paragraph::new(format!("{}kr", price)).aligned(Alignment::Right));
            }
        }
    }

    doc.push(Paragraph::new("–".repeat(24)).aligned(Alignment::Center));
    doc.push({
        let mut tbl = TableLayout::new(vec![1, 1]);
        tbl.row()
            .element(Text::new("Total"))
            .element(Paragraph::new(format!("{}kr", reciept.sum)).aligned(Alignment::Right))
            .push()
            .expect("Couldn't Table Total");
        tbl
    });
    doc.push({
        let mut tbl = TableLayout::new(vec![1, 1]);
        tbl.row()
            .element(Text::new("Betalt via"))
            .element(Paragraph::new(String::from(reciept.payment)).aligned(Alignment::Right))
            .push()
            .expect("Couldn't Table Payment");
        tbl
    });
    doc.push(Break::new(1));
    doc.push(Text::new(format!("{}", time.format("%F %T"))));
    doc.push(Break::new(1));

    doc.render_to_file("genpdf.pdf")
        .expect("Failed to write PDF file");

    Ok(())
}
