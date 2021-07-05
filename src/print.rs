use {
    crate::{error::Result, widgets::Reciept},
    chrono::{DateTime, Local},
    genpdf::{
        elements::{Break, Image, Paragraph},
        fonts, Alignment, Document, Mm, SimplePageDecorator,
    },
    std::io::Cursor,
};

fn pt_to_mm(pt: f64) -> Mm {
    (pt * 0.352_778_f64).into()
}

fn print<M>(_rec: Reciept<M>, _time: DateTime<Local>) -> Result<()> {
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
    doc.set_paper_size((pt_to_mm(8.0 * 72.0), pt_to_mm(11.0 * 72.0)));
    doc.set_page_decorator({
        let mut dec = SimplePageDecorator::new();
        dec.set_margins((10, 10));
        dec
    });

    let logga = Cursor::new(include_bytes!("../../../resources/logga.png"));
    doc.push(Break::new(2));
    doc.push(
        Image::from_reader(logga)
            .unwrap()
            .with_alignment(Alignment::Center),
    );
    doc.push(Paragraph::new("Hello King"));

    doc.render_to_file("genpdf.pdf")
        .expect("Failed to write PDF file");

    Ok(())
}
