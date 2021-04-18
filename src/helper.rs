use iced::{button, Background, Button, Color, Element};

pub struct Clickable;

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
