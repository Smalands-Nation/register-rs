use {
    iced::text_input,
    std::{fmt, str},
};

pub struct NumberInput<N>(text_input::State, Option<N>);

impl<N> NumberInput<N>
where
    N: num_traits::Num + Copy + fmt::Display + str::FromStr + PartialOrd,
{
    pub fn new() -> Self {
        Self(text_input::State::new(), None)
    }

    pub fn build<F, M>(&mut self, min: N, max: N, msg: F) -> iced::TextInput<M>
    where
        N: 'static,
        F: 'static + Fn(Option<N>) -> M,
        M: Clone,
    {
        let clone = self.1.clone();
        iced::TextInput::new(
            &mut self.0,
            "",
            match self.1 {
                Some(n) => {
                    format!("{}", n)
                }
                None => String::new(),
            }
            .as_str(),
            move |s| match N::from_str(s.as_str()) {
                Ok(n) if (min..=max).contains(&n) => msg(Some(n)),
                Err(_) if s.len() == 0 => msg(None),
                _ => msg(clone),
            },
        )
    }

    pub fn update(&mut self, v: Option<N>) {
        self.1 = v;
    }

    pub fn value(&mut self) -> Option<N> {
        self.1
    }
}
