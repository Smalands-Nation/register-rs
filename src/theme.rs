use {
    crate::styles::BORDER_WIDTH,
    iced_aw::style::{badge, card, date_picker, modal, tab_bar},
    iced_native::{
        application,
        overlay::menu,
        widget::{button, checkbox, container, pick_list, rule, scrollable, text, text_input},
        Background, Color,
    },
};

#[derive(Default)]
pub struct Theme(iced_native::Theme);

impl text::StyleSheet for Theme {
    type Style = Option<Color>;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        text::Appearance { color: style }
    }
}

impl checkbox::StyleSheet for Theme {
    type Style = <iced_native::Theme as checkbox::StyleSheet>::Style;

    fn active(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        <iced_native::Theme as checkbox::StyleSheet>::active(self, style, is_checked)
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        <iced_native::Theme as checkbox::StyleSheet>::hovered(self, style, is_checked)
    }
}

impl scrollable::StyleSheet for Theme {
    type Style = <iced_native::Theme as scrollable::StyleSheet>::Style;

    fn active(&self, style: &Self::Style) -> scrollable::style::Scrollbar {
        <iced_native::Theme as scrollable::StyleSheet>::active(self, style)
    }

    fn hovered(&self, style: &Self::Style) -> scrollable::style::Scrollbar {
        <iced_native::Theme as scrollable::StyleSheet>::hovered(self, style)
    }
}

impl text_input::StyleSheet for Theme {
    type Style = <iced_native::Theme as text_input::StyleSheet>::Style;

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        <iced_native::Theme as text_input::StyleSheet>::hovered(&self, style)
    }

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        <iced_native::Theme as text_input::StyleSheet>::active(&self, style)
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        <iced_native::Theme as text_input::StyleSheet>::focused(&self, style)
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        <iced_native::Theme as text_input::StyleSheet>::placeholder_color(&self, style)
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        <iced_native::Theme as text_input::StyleSheet>::value_color(&self, style)
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        <iced_native::Theme as text_input::StyleSheet>::selection_color(&self, style)
    }
}

impl tab_bar::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: Self::Style, is_active: bool) -> tab_bar::Appearance {
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
        }
    }

    fn hovered(&self, style: Self::Style, is_active: bool) -> tab_bar::Appearance {
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
//default iced_native::Theme impl
impl std::ops::Deref for Theme {
    type Target = iced_native::Theme;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! impl_via_deref {
    ($path:tt, $(&$fn:ident),+) => {
        impl $path::StyleSheet for Theme {
            type Style = <iced_native::Theme as $path::StyleSheet>::Style;

            $(fn $fn(&self, style: &Self::Style) -> $path::Appearance {
                <iced_native::Theme as $path::StyleSheet>::$fn(self, style)
            })*
        }
    };

    ($path:tt, $($fn:ident),+) => {
        impl $path::StyleSheet for Theme {
            type Style = <iced_native::Theme as $path::StyleSheet>::Style;

            $(fn $fn(&self, style: Self::Style) -> $path::Appearance {
                <iced_native::Theme as $path::StyleSheet>::$fn(self, style)
            })*
        }
    };
}

impl_via_deref! {application, &appearance}
impl_via_deref! {button, &active}
impl_via_deref! {container, &appearance}
impl_via_deref! {pick_list, &active, &hovered}
impl_via_deref! {rule, &appearance}
impl_via_deref! {menu, &appearance}

impl_via_deref! {badge, active}
impl_via_deref! {card, active}
impl_via_deref! {date_picker, active, selected, hovered, focused}
impl_via_deref! {modal, active}
