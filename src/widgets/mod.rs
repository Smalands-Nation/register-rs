pub mod calc;
pub mod numberinput;
pub mod square_button;

pub use {numberinput::NumberInput, square_button::SquareButton};

#[allow(non_camel_case_types)]
pub type BIG_TEXT = frost::text::Text<45>;
#[allow(non_camel_case_types)]
pub type SMALL_TEXT = frost::text::Text<20>;

macro_rules! _column {
    ($($elem:expr),* $(,)?) => {
        ::iced::widget::column!( $($elem),*).height(::iced::Length::Fill)
    };
}
macro_rules! _row {
    ($($elem:expr),* $(,)?) => {
        ::iced::widget::row!( $($elem),*).width(::iced::Length::Fill)
    };
}

macro_rules! padded_column {
    ($($elem:expr),+ $(,)?) => {
        $crate::widgets::column!( $($elem),*)
            .spacing($crate::theme::DEF_PADDING)
            .padding($crate::theme::DEF_PADDING)
    };
}
macro_rules! padded_row {
    ($($elem:expr),+ $(,)?) => {
        $crate::widgets::row!( $($elem),*)
            .spacing($crate::theme::DEF_PADDING)
            .padding($crate::theme::DEF_PADDING)
    };
}

//NOTE reexport with renames to avoid conflict with std
pub(crate) use {_column as column, _row as row, padded_column, padded_row};
