pub mod calc;
pub mod grid;
pub mod numberinput;
pub mod square_button;

pub use {grid::Grid, numberinput::NumberInput, square_button::SquareButton};

#[allow(non_camel_case_types)]
pub type BIG_TEXT = frost::text::Text<45>;
#[allow(non_camel_case_types)]
pub type SMALL_TEXT = frost::text::Text<20>;

macro_rules! _column {
    (#nopad $($elem:expr),+ $(,)?) => {
        ::iced::widget::Column::with_children(vec![$($elem.into()),*])
    };
    ($($elem:expr),+ $(,)?) => {
        $crate::widgets::column!(#nopad $($elem),*)
            .spacing($crate::theme::DEF_PADDING)
            .padding($crate::theme::DEF_PADDING)
    };
}
macro_rules! _row {
    (#nopad $($elem:expr),+ $(,)?) => {
        //::iced::widget::Row::with_children(vec![$($elem.into()),*])
        ::iced::widget::Row::with_children(vec![$($crate::Element::from($elem)),*])
    };
    ($($elem:expr),+ $(,)?) => {
        $crate::widgets::row!(#nopad $($elem),*)
            .spacing($crate::theme::DEF_PADDING)
            .padding($crate::theme::DEF_PADDING)
    };
}

//NOTE reexport with renames to avoid conflict with std
pub(crate) use {_column as column, _row as row};
