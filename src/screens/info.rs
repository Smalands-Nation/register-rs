use {
    super::Screen,
    crate::{
        command,
        icons::Icon,
        item::Item,
        sql,
        styles::{DEF_PADDING, RECEIPT_WIDTH, SMALL_TEXT},
        widgets::{Grid, NumberInput, SquareButton},
    },
    iced::{
        widget::{Column, Container, Row, Rule, Scrollable, Text, TextInput},
        Alignment, Command, Element, Length, Space,
    },
    iced_aw::{style::badge, Badge, Card},
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

    #[cfg(not(debug_assertions))]
    fn update(&mut self, msg: Self::InMessage) -> Command<Self::ExMessage> {
        match msg {
            Message::Updated(ver) => self.new_version = Some(ver),
            _ => (),
        }
        Command::none()
    }

    #[cfg(debug_assertions)]
    fn update(&mut self, _: Self::InMessage) -> Command<Self::ExMessage> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::ExMessage> {
        Column::new()
            .push(
                Container::new(
                    Column::with_children(vec![
                        Row::new()
                            .push(Text::new("Smålands_register version"))
                            .push(
                                Badge::new(Text::new(self.current))
                                    .style(badge::Info)
                                    .padding(DEF_PADDING),
                            )
                            .spacing(DEF_PADDING)
                            .align_items(Alignment::Center)
                            .into(),
                        match &self.new_version {
                            Some(ver) => Row::new()
                                .push(Text::new("Ny version"))
                                .push(
                                    Badge::new(Text::new(ver))
                                        .style(badge::Warning)
                                        .padding(DEF_PADDING),
                                )
                                .push(Text::new("installeras vid omstart."))
                                .spacing(DEF_PADDING)
                                .align_items(Alignment::Center)
                                .into(),
                            None => Badge::new(Text::new("Detta är senaste versionen."))
                                .padding(DEF_PADDING)
                                .style(badge::Success)
                                .into(),
                        },
                    ])
                    .spacing(DEF_PADDING),
                )
                .center_x()
                .center_y()
                .width(Length::Fill)
                .height(Length::Fill),
            )
            .push(
                Text::new("Programmerad av Axel Paulander (Styrelse 20/21 & 21/22)")
                    .size(SMALL_TEXT),
            )
            .push(
                Text::new("All kod är tillänglig på github.com/Smalands-Nation/register-rs")
                    .size(SMALL_TEXT),
            )
            .align_items(Alignment::Center)
            .into()
    }
}
