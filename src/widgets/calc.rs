use {
    super::{column, row, SquareButton},
    crate::{
        icons::Icon,
        theme::{DEF_PADDING, DEF_TEXT, SQUARE_BUTTON},
    },
    frost::wrap::{Direction, Wrap},
    iced::{
        alignment::{Alignment, Horizontal},
        widget::{Component, Rule, Space, Text},
        Element, Length,
    },
};

pub struct Calc<'a, M> {
    multi: u32,
    on_set: Box<dyn Fn(u32) -> M + 'a>,
}

#[derive(Debug, Clone)]
pub enum Event {
    Clear,
    Update(u32),
    Save,
}

impl<'a, M> Calc<'a, M> {
    pub fn new<F>(multi: u32, on_set: F) -> Self
    where
        F: Fn(u32) -> M + 'a,
    {
        Self {
            multi: if multi > 0 { multi } else { 1 },
            on_set: Box::new(on_set),
        }
    }
}

impl<'a, M> Component<M> for Calc<'a, M> {
    type State = u32;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Event) -> Option<M> {
        match event {
            Event::Clear if *state == 0 => {
                self.multi = 1;
                Some((self.on_set)(1))
            }
            Event::Clear if *state != 0 => {
                *state = 0;
                None
            }
            Event::Update(v) if (v, *state) != (0, 0) => {
                *state = match *state * 10 + v {
                    0 => 1,
                    v @ 1..=100 => v,
                    _ => 100,
                };
                None
            }
            Event::Save if *state != 0 => {
                self.multi = *state;
                *state = 0;
                Some((self.on_set)(self.multi))
            }
            _ => None,
        }
    }

    fn view(&self, state: &Self::State) -> Element<Event> {
        column![
            row![
                Text::new(format!("{:>3}x", self.multi)).horizontal_alignment(Horizontal::Left),
                Rule::vertical(DEF_PADDING),
                Text::new(if *state != 0 {
                    format!("{state}")
                } else {
                    String::new()
                })
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Right),
            ]
            .height(Length::Shrink)
            .width(Length::Fill),
            Space::with_height(Length::Fixed(DEF_PADDING as f32)),
            Wrap::with_children(
                Direction::Row(3),
                (0..12)
                    .map(|i| {
                        match i {
                            0..=8 => SquareButton::text(format!("{}", i + 1))
                                .on_press(Event::Update(i as u32 + 1)),
                            9 => SquareButton::text("c").on_press(Event::Clear),
                            10 => SquareButton::text("0").on_press(Event::Update(0)),
                            _ => SquareButton::icon(Icon::Cross).on_press(Event::Save),
                        }
                        .into()
                    })
                    .collect(),
            )
            .spacing(DEF_PADDING),
        ]
        .height(Length::Shrink)
        .align_items(Alignment::Center)
        .into()
    }
}

impl<'a, M> From<Calc<'a, M>> for Element<'a, M>
where
    M: 'a,
{
    fn from(calc: Calc<'a, M>) -> Self {
        iced::widget::component(calc)
    }
}
