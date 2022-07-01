pub mod calc;
pub mod grid;
pub mod numberinput;
pub mod square_button;

pub use {grid::Grid, numberinput::NumberInput, square_button::SquareButton};

#[allow(non_camel_case_types)]
pub type BIG_TEXT = frost::pure::Text<45>;
#[allow(non_camel_case_types)]
pub type SMALL_TEXT = frost::pure::Text<20>;
