use {
    iced::{
        application, color,
        overlay::menu,
        widget::{button, checkbox, container, pick_list, rule, scrollable, text, text_input},
        Background, BorderRadius, Color,
    },
    iced_aw::style::{badge, card, date_picker, modal, tab_bar},
};

const BORDER_RADIUS: f32 = 2.0;
pub const BORDER_WIDTH: f32 = 2.0;

pub const DEF_TEXT: f32 = 35.0;

pub const DEF_PADDING: u16 = 10;
pub const SMALL_PADDING: u16 = 5;

pub const RECEIPT_WIDTH: f32 = 300.0;
pub const SQUARE_BUTTON: f32 = 15.0 + crate::widgets::BIG_TEXT::size() as f32;

#[derive(Default)]
pub struct Theme(iced::Theme);

impl text::StyleSheet for Theme {
    type Style = Option<Color>;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        text::Appearance { color: style }
    }
}

#[derive(Default)]
pub enum Container {
    #[default]
    Empty,
    Border,
    Fill(Color),
    BorderFill(Color),
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::BLACK),
            background: match *style {
                Container::Fill(bg) | Container::BorderFill(bg) => Some(Background::Color(bg)),
                _ => None,
            },
            border_radius: match style {
                Container::Border | Container::BorderFill(_) => BORDER_RADIUS,
                _ => 0.0,
            }
            .into(),
            border_width: match style {
                Container::Border | Container::BorderFill(_) => BORDER_WIDTH,
                _ => 0.0,
            },
            border_color: Color::BLACK,
        }
    }
}

impl button::StyleSheet for Theme {
    type Style = Container;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: Color::BLACK,
            background: match *style {
                Container::Fill(bg) | Container::BorderFill(bg) => Some(Background::Color(bg)),
                _ => None,
            },
            border_radius: match style {
                Container::Border | Container::BorderFill(_) => BORDER_RADIUS,
                _ => 0.0,
            }
            .into(),
            border_width: match style {
                Container::Border | Container::BorderFill(_) => BORDER_WIDTH,
                _ => 0.0,
            },
            border_color: Color::BLACK,
            shadow_offset: Default::default(),
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        Self::active(self, style)
    }
}

impl checkbox::StyleSheet for Theme {
    type Style = <iced::Theme as checkbox::StyleSheet>::Style;

    fn active(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: Background::Color(Color::WHITE),
            icon_color: Color::BLACK,
            border_radius: BORDER_RADIUS.into(),
            border_width: BORDER_WIDTH,
            border_color: Color::BLACK,
            text_color: None,
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: Background::Color(color!(0xD0D0D0)),
            icon_color: Color::BLACK,
            border_radius: BORDER_RADIUS.into(),
            border_width: BORDER_WIDTH,
            border_color: Color::BLACK,
            text_color: None,
        }
    }
}

impl scrollable::StyleSheet for Theme {
    type Style = <iced::Theme as scrollable::StyleSheet>::Style;

    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        <iced::Theme as scrollable::StyleSheet>::active(self, style)
    }

    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> scrollable::Scrollbar {
        <iced::Theme as scrollable::StyleSheet>::hovered(self, style, is_mouse_over_scrollbar)
    }
}

impl text_input::StyleSheet for Theme {
    type Style = <iced::Theme as text_input::StyleSheet>::Style;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        <iced::Theme as text_input::StyleSheet>::active(self, style)
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        <iced::Theme as text_input::StyleSheet>::focused(self, style)
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        <iced::Theme as text_input::StyleSheet>::placeholder_color(self, style)
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        <iced::Theme as text_input::StyleSheet>::value_color(self, style)
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        <iced::Theme as text_input::StyleSheet>::selection_color(self, style)
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        <iced::Theme as text_input::StyleSheet>::disabled_color(self, style)
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        <iced::Theme as text_input::StyleSheet>::disabled(self, style)
    }
}

impl tab_bar::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style, is_active: bool) -> tab_bar::Appearance {
        tab_bar::Appearance {
            background: None,
            border_color: Some(Color::TRANSPARENT),
            border_width: BORDER_WIDTH * 2.0,
            tab_label_background: if is_active {
                Background::Color(Color::WHITE)
            } else {
                Background::Color([0.8, 0.8, 0.8].into())
            },
            tab_label_border_color: Color::TRANSPARENT,
            tab_label_border_width: BORDER_WIDTH,
            icon_color: Color::BLACK,
            text_color: Color::BLACK,
            ..tab_bar::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> tab_bar::Appearance {
        tab_bar::Appearance {
            tab_label_background: if is_active {
                Background::Color(Color::WHITE)
            } else {
                Background::Color([0.9, 0.9, 0.9].into())
            },
            ..self.active(style, is_active)
        }
    }
}

//HACK using the dark magic of Deref and macros we can auto-impl all StyleSheet traits that use the
//default iced::Theme impl
impl std::ops::Deref for Theme {
    type Target = iced::Theme;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! impl_via_deref {
    ($path:tt, $($fn:ident),+) => {
        impl $path::StyleSheet for Theme {
            type Style = <iced::Theme as $path::StyleSheet>::Style;

            $(fn $fn(&self, style: &Self::Style) -> $path::Appearance {
                <iced::Theme as $path::StyleSheet>::$fn(self, style)
            })*
        }
    };
}

impl_via_deref! {application, appearance}
impl_via_deref! {pick_list, active, hovered}
impl_via_deref! {rule, appearance}
impl_via_deref! {menu, appearance}

impl_via_deref! {badge, active}
impl_via_deref! {card, active}
impl_via_deref! {date_picker, active, selected, hovered, focused}
impl_via_deref! {modal, active}
