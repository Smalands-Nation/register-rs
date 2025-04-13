use {
    super::Receipt,
    genpdf::{
        Alignment, Document, SimplePageDecorator,
        elements::{Break, Image, Paragraph, TableLayout, Text},
        fonts,
    },
    std::{
        io::Cursor,
        path::PathBuf,
        sync::{Arc, LazyLock, OnceLock},
    },
};

pub(crate) static RECEIPT_PATH: OnceLock<PathBuf> = OnceLock::new();

pub(crate) static FONT: LazyLock<fonts::FontData> = LazyLock::new(|| {
    fonts::FontData::new(
        include_bytes!("../../../resources/IBMPlexMono-Regular.ttf").to_vec(),
        None,
    )
    .unwrap()
});

fn create_pdf(path: impl Into<PathBuf>, receipt: &Receipt) -> Result<PathBuf> {
    let mut doc = Document::new(fonts::FontFamily {
        regular: FONT.clone(),
        bold: FONT.clone(),
        italic: FONT.clone(),
        bold_italic: FONT.clone(),
    });
    doc.set_paper_size((72, 300));
    doc.set_page_decorator({
        let mut dec = SimplePageDecorator::new();
        dec.set_margins((10, 5));
        dec
    });

    let logga = Cursor::new(include_bytes!("../../../resources/logga.png"));
    doc.push(
        Image::from_reader(logga)
            .unwrap()
            .with_alignment(Alignment::Center),
    );

    doc.push(Paragraph::new("Smålands Nation").aligned(Alignment::Center));
    doc.push(Paragraph::new("Nyhemsgatan 30").aligned(Alignment::Center));
    doc.push(Paragraph::new("302 49 Halmstad").aligned(Alignment::Center));
    doc.push(Paragraph::new("–".repeat(24)).aligned(Alignment::Center));

    for (item, amount) in receipt.items.iter() {
        doc.push(Text::new(item.name().clone()));
        if item.is_special() {
            doc.push(
                Paragraph::new(format!("{}kr", item.price() * amount)).aligned(Alignment::Right),
            );
        } else {
            doc.push({
                let mut tbl = TableLayout::new(vec![1, 1]);
                tbl.row()
                    .element(Text::new(format!("{}x{}kr", amount, item.price())))
                    .element(
                        Paragraph::new(format!("{}kr", item.price() * amount))
                            .aligned(Alignment::Right),
                    )
                    .push()
                    .expect("Couldn't Table Price");
                tbl
            });
        }
    }

    doc.push(Paragraph::new("–".repeat(24)).aligned(Alignment::Center));
    doc.push({
        let mut tbl = TableLayout::new(vec![1, 1]);
        tbl.row()
            .element(Text::new("Total"))
            .element(Paragraph::new(format!("{}kr", receipt.sum())).aligned(Alignment::Right))
            .push()
            .expect("Couldn't Table Total");
        tbl
    });
    doc.push({
        let mut tbl = TableLayout::new(vec![1, 1]);
        tbl.row()
            .element(Text::new("Betalt via"))
            .element(Paragraph::new(receipt.payment.to_string()).aligned(Alignment::Right))
            .push()
            .expect("Couldn't Table Payment");
        tbl
    });
    doc.push(Break::new(1));
    doc.push(Text::new(format!("{}", receipt.time.format("%F %T"))));
    doc.push(Break::new(1));

    let mut path = path.into();
    path.push(format!("receipt_{}.pdf", receipt.time.format("%F_%T")).replace(':', "-"));
    doc.render_to_file(path.clone())
        .map_err(|e| Error::Pdf(Arc::new(e)))?;

    Ok(path)
}

#[cfg(target_os = "windows")]
pub async fn print(receipt: &Receipt) -> Result<()> {
    let filename = create_pdf(RECEIPT_PATH.get().ok_or(Error::NoPrintPath)?, &receipt)?;
    let mut pdf_to_printer = dirs::config_dir().ok_or(Error::NoConfPath)?;
    pdf_to_printer.push("smaland_register");
    pdf_to_printer.push("PDFtoPrinter.exe");
    if std::process::Command::new(pdf_to_printer)
        .args([filename])
        .output()
        .map_err(|e| Error::Io(e.kind()))?
        .status
        .success()
    {
        Ok(())
    } else {
        Err(Error::PrintFailed)?
    }
}

#[cfg(not(target_os = "windows"))]
pub async fn print(receipt: &Receipt) -> Result<()> {
    let filename = create_pdf(RECEIPT_PATH.get().ok_or(Error::NoPrintPath)?, receipt)?;
    println!("{:?}", filename.display());
    if std::process::Command::new("/usr/bin/lp")
        .args([filename])
        .output()
        .map_err(|e| Error::Io(e.kind()))?
        .status
        .success()
    {
        Ok(())
    } else {
        Err(Error::PrintFailed)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    PrintFailed,
    NoPrintPath,
    NoConfPath,
    Io(std::io::ErrorKind),
    Pdf(Arc<genpdf::error::Error>),
}
