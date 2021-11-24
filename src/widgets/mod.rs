//pub mod buttonscroller;
pub mod calc;
pub mod clickable;
pub mod grid;
pub mod numberinput;
pub mod receipt;
pub mod square_button;
pub mod textinput;

pub use {
    /*buttonscroller::ButtonScroller,*/ clickable::Clickable, grid::Grid,
    numberinput::NumberInput, receipt::Receipt, square_button::SquareButton, textinput::TextInput,
};

/*
use iced::Element;
pub trait Widget<M> {
    fn view(&mut self) -> Element<M>;
}

impl<'a, M> Widget<M> for Element<'a, M> {
    fn view(&mut self) -> Element<M> {
        *self
    }
}*/
