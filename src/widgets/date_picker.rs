use {
    crate::styles::DEF_PADDING,
    chrono::{Datelike, Local},
    iced::{
        button::{self, Button},
        Text,
    },
    iced_aw::native::date_picker::{self, Date, State},
};

pub struct DatePicker {
    pub state: State,
    date: Date,
    bttn_state: button::State,
}

impl DatePicker {
    pub fn new() -> Self {
        let date = Local::today().naive_local();
        let mut state = State::now();
        state.set_date(date.year(), date.month(), date.day());

        Self {
            state,
            date: date.into(),
            bttn_state: button::State::new(),
        }
    }

    pub fn build<F, M>(&mut self, open: M, cancel: M, submit: F) -> impl Into<iced::Element<M>>
    where
        F: 'static + Fn(Date) -> M,
        M: 'static + Clone,
    {
        date_picker::DatePicker::new(
            &mut self.state,
            Button::new(&mut self.bttn_state, Text::new(format!("{}", self.date)))
                .on_press(open)
                .padding(DEF_PADDING),
            cancel,
            submit,
        )
    }

    pub fn update(&mut self, d: Date) {
        self.date = d;
    }

    pub fn value(&mut self) -> Date {
        self.date.clone()
    }
}
