use iced::{Column, Element, Length, Row};

pub struct Grid<'a, Message> {
    rows: u32,
    cols: u32,
    width: Length,
    height: Length,
    spacing: u16,
    padding: u16,
    max_height: u32,
    max_width: u32,
    children: Vec<Element<'a, Message>>,
}

impl<'a, Message> Grid<'a, Message> {
    pub fn new(rows: u32, cols: u32) -> Self {
        Self::with_children(rows, cols, Vec::new())
    }

    pub fn with_children(rows: u32, cols: u32, children: Vec<Element<'a, Message>>) -> Self {
        Self {
            rows,
            cols,
            width: Length::Shrink,
            height: Length::Shrink,
            spacing: 0,
            padding: 0,
            max_height: u32::MAX,
            max_width: u32::MAX,
            children,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn spacing(mut self, units: u16) -> Self {
        self.spacing = units;
        self
    }

    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn push<E>(mut self, child: E) -> Self
    where
        E: Into<Element<'a, Message>>,
    {
        self.children.push(child.into());
        self
    }
}

impl<'a, Message: 'a> From<Grid<'a, Message>> for Element<'a, Message> {
    fn from(g: Grid<'a, Message>) -> Self {
        let mut col = Column::new()
            .width(g.width)
            .height(g.height)
            .padding(g.padding)
            .spacing(g.spacing)
            .max_height(g.max_height)
            .max_width(g.max_width);
        let mut r = Row::new()
            .spacing(g.spacing)
            .max_height(g.max_height / g.rows);
        let mut i = 0;
        for child in g.children {
            r = r.push(child);
            i += 1;
            if i == g.cols {
                col = col.push(r);
                r = Row::new()
                    .spacing(g.spacing)
                    .max_height(g.max_height / g.rows);
                i = 0;
            }
        }
        col.push(r).into()
    }
}
