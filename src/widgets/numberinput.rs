use {
    iced::widget::TextInput,
    std::{fmt, ops, str},
};

//TODO -> Component
pub struct NumberInput<N>(Option<N>);

impl<N> NumberInput<N>
where
    N: num_traits::Num + Copy + fmt::Display + str::FromStr + PartialOrd,
{
    pub fn new() -> Self {
        Self(None)
    }

    pub fn build<'a, R, F, M>(&'a self, range: R, msg: F) -> TextInput<'a, M>
    where
        R: 'a + ops::RangeBounds<N>,
        F: 'a + Fn(Option<N>) -> M,
        M: Clone,
    {
        TextInput::new(
            "",
            match self.0 {
                Some(n) => {
                    format!("{}", n)
                }
                None => String::new(),
            }
            .as_str(),
            move |s| match s.parse() {
                Ok(n) if range.contains(&n) => msg(Some(n)),
                Err(_) if s.is_empty() => msg(None),
                _ => msg(self.0),
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
