use {
    super::Screen,
    crate::{
        command,
        icons::Icon,
        item::Item,
        sql,
        styles::{BIG_TEXT, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{Grid, NumberInput, SquareButton},
    },
    iced::{
        pure::{
            widget::{Button, Checkbox, Column, Row, Rule, Scrollable, Text, TextInput},
            Pure, State,
        },
        Alignment, Command, Element, Length, Space,
    },
    iced_aw::{
        modal::{self, Modal},
        Card,
    },
    rusqlite::params,
};

#[cfg(not(debug_assertions))]
pub type Message = self_update::Status;

#[cfg(debug_assertions)]
#[derive(Debug, Clone)]
pub struct Message();

pub struct Info {
    current: &'static str,
    new_version: Option<String>,
}

impl Screen for Info {
    type ExMessage = super::Message;
    type InMessage = Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                current: self_update::cargo_crate_version!(),
                new_version: None,
            },
            command!(crate::config::update()),
        )
    }

    fn update(&mut self, msg: Self::InMessage) -> Command<Self::ExMessage> {
        #[cfg(not(debug_assertions))]
        match msg {
            Message::Updated(ver) => self.new_version = Some(ver),
            _ => (),
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::ExMessage> {
        Text::new(self.current).into()
    }
}
