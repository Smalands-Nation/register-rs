use iced::{button, container};

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

pub struct Style(pub container::Style);

impl container::StyleSheet for Style {
    fn style(&self) -> container::Style {
        self.0
    }
}
