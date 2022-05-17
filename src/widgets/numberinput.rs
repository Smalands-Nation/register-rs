use {
    iced::pure::widget::TextInput,
    std::{fmt, str},
};

pub struct NumberInput<N>(Option<N>);

impl<N> NumberInput<N>
where
    N: num_traits::Num + Copy + fmt::Display + str::FromStr + PartialOrd,
{
    pub fn new() -> Self {
        Self(None)
    }

    pub fn build<F, M>(&mut self, min: N, max: N, msg: F) -> TextInput<M>
    where
        N: 'static,
        F: 'static + Fn(Option<N>) -> M,
        M: Clone,
    {
        let clone = self.0;
        TextInput::new(
            "",
            match self.0 {
                Some(n) => {
                    format!("{}", n)
                }
                None => String::new(),
            }
            .as_str(),
            move |s| match N::from_str(s.as_str()) {
                Ok(n) if (min..=max).contains(&n) => msg(Some(n)),
                Err(_) if s.is_empty() => msg(None),
                _ => msg(clone),
            },
        )
    }

    pub fn update(&mut self, v: Option<N>) {
        self.0 = v;
    }

    pub fn value(&mut self) -> Option<N> {
        self.0
    }
}
