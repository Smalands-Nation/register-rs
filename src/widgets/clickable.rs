use iced::pure::{widget::button, Element};

pub struct Clickable;

impl Clickable {
    pub fn new<'a, M, E>(e: E) -> button::Button<'a, M>
    where
        M: Clone,
        E: Into<Element<'a, M>>,
    {
        button::Button::new(e).style(Self)
    }
}

impl button::StyleSheet for Clickable {
    fn active(&self) -> button::Style {
        button::Style::default()
    }

    fn hovered(&self) -> button::Style {
        self.active()
    }

    fn pressed(&self) -> button::Style {
        self.active()
    }

    fn disabled(&self) -> button::Style {
        self.active()
    }
}
