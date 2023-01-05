use {
    super::Screen,
    crate::{
        command,
        styles::DEF_PADDING,
        widgets::{column, row, SMALL_TEXT},
    },
    iced::{
        widget::{Container, Text},
        Alignment, Command, Element, Length,
    },
    iced_aw::{style::badge::BadgeStyles, Badge},
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

    fn view(&self) -> Element<Self::ExMessage> {
        column![
            #nopad
            Container::new(
                column![
                    row![
                        Text::new("Smålands_register version"),
                        Badge::new(Text::new(self.current))
                            .style(BadgeStyles::Info)
                            .padding(DEF_PADDING),
                    ]
                    .align_items(Alignment::Center),
                    match &self.new_version {
                        Some(ver) => row![
                            Text::new("Ny version"),
                            Badge::new(Text::new(ver))
                                .style(BadgeStyles::Warning)
                                .padding(DEF_PADDING),
                            Text::new("installeras vid omstart."),
                        ],
                        None => row![
                            Text::new("Dettta är"),
                            Badge::new(Text::new("Senaste versionen."))
                                .style(BadgeStyles::Success)
                                .padding(DEF_PADDING),
                        ],
                    }
                    .align_items(Alignment::Center),
                ]
                .align_items(Alignment::Center)
                .spacing(DEF_PADDING),
            )
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill),
            SMALL_TEXT::new("Programmerad av Axel Paulander (Styrelse 20/21 & 21/22)",),
            SMALL_TEXT::new("All kod är tillänglig på github.com/Smalands-Nation/register-rs",),
        ]
        .align_items(Alignment::Center)
        .into()
    }
}
