use {
    crate::theme::DEF_PADDING,
    iced::{
        widget::{Component, TextInput},
        Element,
    },
    std::{cmp::PartialOrd, fmt::Display, ops::RangeInclusive, str::FromStr},
};

pub struct NumberInput<'a, N, M> {
    on_change: Box<dyn Fn(N) -> M + 'a>,
    range: RangeInclusive<N>,
    state: Option<N>,
}

impl<'a, N, M> NumberInput<'a, N, M> {
    pub fn new<F>(range: RangeInclusive<N>, on_change: F, value: N) -> Self
    where
        F: Fn(N) -> M + 'a,
    {
        Self {
            on_change: Box::new(on_change),
            range,
            state: Some(value),
        }
    }
}

#[derive(Clone)]
pub enum Event {
    Input(String),
}

impl<N, M> Component<M> for NumberInput<'_, N, M>
where
    N: Display + FromStr + Default + PartialOrd + Copy,
    M: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<M> {
        match event {
            Event::Input(s) => {
                if s.is_empty() {
                    self.state = None;
                    None
                } else {
                    let n: N = s.parse().ok()?;
                    self.range.contains(&n).then(|| {
                        self.state = Some(n);
                        (self.on_change)(n)
                    })
                }
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        TextInput::new(
            "",
            &self
                .state
                .map(|s| s.to_string())
                .or(self.state.map(|n| n.to_string()))
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
