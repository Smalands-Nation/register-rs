use {
    crate::{
        error::Result,
        receipt::{Item, Receipt},
    },
    chrono::{DateTime, Local},
    genpdf::{
        elements::{Break, Image, Paragraph, TableLayout, Text},
        fonts, Alignment, Document, SimplePageDecorator,
    },
    std::io::Cursor,
};

async fn create_pdf(receipt: &Receipt, time: DateTime<Local>) -> Result<String> {
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
    doc.set_paper_size((72, 300));
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

    for item in receipt.items.values() {
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
            .element(Paragraph::new(format!("{}kr", receipt.sum)).aligned(Alignment::Right))
            .push()
            .expect("Couldn't Table Total");
        tbl
    });
    doc.push({
        let mut tbl = TableLayout::new(vec![1, 1]);
        tbl.row()
            .element(Text::new("Betalt via"))
            .element(Paragraph::new(String::from(receipt.payment)).aligned(Alignment::Right))
            .push()
            .expect("Couldn't Table Payment");
        tbl
    });
    doc.push(Break::new(1));
    doc.push(Text::new(format!("{}", time.format("%F %T"))));
    doc.push(Break::new(1));

    let filename = format!("receipt_{}.pdf", time.format("%F_%T")).replace(":", "-");
    doc.render_to_file(filename.clone())
        .expect("Failed to write PDF file");

    Ok(filename)
}

#[cfg(target_os = "windows")]
pub async fn print(receipt: Receipt, time: DateTime<Local>) -> Result<Receipt> {
    let filename = create_pdf(&receipt, time).await?;
    let mut print_to_pdf = dirs::config_dir().ok_or("No config path")?;
    print_to_pdf.push("smaland_register");
    print_to_pdf.push("PDFtoPrinter.exe");
    if std::process::Command::new(print_to_pdf)
        .args([filename])
        .output()
        .map_err(|e| e.kind())?
        .status
        .success()
    {
        Ok(receipt)
    } else {
        Err("Print failed")?
    }
}

#[cfg(not(target_os = "windows"))]
pub async fn print(receipt: Receipt, time: DateTime<Local>) -> Result<Receipt> {
    let filename = create_pdf(&receipt, time).await?;
    if std::process::Command::new("/usr/bin/lp")
        .args([filename])
        .output()
        .map_err(|e| e.kind())?
        .status
        .success()
    {
        Ok(receipt)
    } else {
        Err("Print failed")?
    }
}
