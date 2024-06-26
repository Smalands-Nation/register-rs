use {
    crate::{
        theme::DEF_PADDING,
        widgets::{column, padded_column, padded_row, SMALL_TEXT},
    },
    iced::{
        widget::{Component, Container, Text},
        Alignment, Element, Length,
    },
    iced_aw::{style::badge::BadgeStyles, Badge},
    self_update::{cargo_crate_version, Status},
};

pub struct Info {
    current: &'static str,
    status: Status,
}

impl Info {
    pub fn new(status: Status) -> Self {
        Self {
            current: cargo_crate_version!(),
            status,
        }
    }
}

impl<M> Component<M> for Info {
    type State = ();
    type Event = ();

    fn update(&mut self, _: &mut Self::State, _: Self::Event) -> Option<M> {
        None
    }

    fn view(&self, _: &Self::State) -> Element<Self::Event> {
        column![
            Container::new(
                padded_column![
                    padded_row![
                        Text::new("Smålands_register version"),
                        Badge::new(Text::new(self.current))
                            .style(BadgeStyles::Info)
                            .padding(DEF_PADDING),
                    ]
                    .width(Length::Shrink)
                    .align_items(Alignment::Center),
                    match &self.status {
                        Status::Updated(ver) => padded_row![
                            Text::new("Ny version"),
                            Badge::new(Text::new(ver))
                                .style(BadgeStyles::Warning)
                                .padding(DEF_PADDING),
                            Text::new("installeras vid omstart."),
                        ],
                        Status::UpToDate(_) => padded_row![
                            Text::new("Detta är"),
                            Badge::new(Text::new("Senaste versionen."))
                                .style(BadgeStyles::Success)
                                .padding(DEF_PADDING),
                        ],
                    }
                    .width(Length::Shrink)
                    .align_items(Alignment::Center),
                ]
                .height(Length::Shrink)
                .align_items(Alignment::Center)
                .spacing(DEF_PADDING),
            )
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill),
            SMALL_TEXT::new("All kod är tillänglig på github.com/Smalands-Nation/register-rs",),
        ]
        .align_items(Alignment::Center)
        .into()
    }
}

impl<'a, M> From<Info> for Element<'a, M>
where
    M: 'a,
{
    fn from(info: Info) -> Self {
        iced::widget::component(info)
    }
}
