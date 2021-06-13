use iced::text_input;

pub struct TextInput(text_input::State, String);

impl TextInput {
    pub fn new() -> Self {
        Self(text_input::State::new(), String::new())
    }

    pub fn build<F, M>(&mut self, placeholder: &str, msg: F) -> iced::TextInput<M>
    where
        F: 'static + Fn(String) -> M,
        M: Clone,
    {
        iced::TextInput::new(&mut self.0, placeholder, &self.1, msg)
    }

    pub fn update(&mut self, s: String) {
        self.1 = s;
    }

    pub fn value(&mut self) -> String {
        self.1.clone()
    }
}
