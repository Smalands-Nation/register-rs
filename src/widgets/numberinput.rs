use {
    crate::{theme::DEF_PADDING, Element, Renderer},
    iced::widget::{Component, TextInput},
    std::{cmp::PartialOrd, fmt::Display, ops::RangeInclusive, str::FromStr},
};

pub struct NumberInput<'a, N, M> {
    on_change: Box<dyn Fn(N) -> M + 'a>,
    range: RangeInclusive<N>,
    initial_value: Option<N>,
}

impl<'a, N, M> NumberInput<'a, N, M> {
    pub fn new<F>(range: RangeInclusive<N>, on_change: F, initial_value: Option<N>) -> Self
    where
        F: Fn(N) -> M + 'a,
    {
        Self {
            on_change: Box::new(on_change),
            range,
            initial_value,
        }
    }
}

#[derive(Clone)]
pub enum Event {
    Input(String),
}

impl<'a, N, M> Component<M, Renderer> for NumberInput<'a, N, M>
where
    N: Display + FromStr + Default + PartialOrd + Copy,
    M: Clone,
{
    type State = Option<N>;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<M> {
        self.initial_value = None;
        match event {
            Event::Input(s) => {
                if s.is_empty() {
                    *state = None;
                    None
                } else {
                    let n: N = s.parse().ok()?;
                    self.range.contains(&n).then(|| {
                        *state = Some(n);
                        (self.on_change)(n)
                    })
                }
            }
        }
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event> {
        TextInput::new(
            "",
            &state
                .map(|s| s.to_string())
                .or(self.initial_value.map(|n| n.to_string()))
                .unwrap_or_default(),
        )
        .on_input(Event::Input)
        .padding(DEF_PADDING)
        .into()
    }
}

impl<'a, N, M> From<NumberInput<'a, N, M>> for Element<'a, M>
where
    N: Display + FromStr + Default + PartialOrd + Copy + 'static,
    M: Clone + 'a,
{
    fn from(calc: NumberInput<'a, N, M>) -> Self {
        iced::widget::component(calc)
    }
}
