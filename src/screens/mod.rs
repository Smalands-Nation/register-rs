mod menu;

pub use menu::Menu;
use {giftwrap::Wrap, iced::Element};

#[derive(Debug, Clone)]
pub enum Message {
    None,
    SwapTab(usize),
    Menu(menu::Message),
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Wrap)]
pub enum Screen {
    Menu(Menu),
}

impl Screen {
    pub fn view(&mut self) -> Element<Message> {
        match self {
            Self::Menu(m) => m.view().map(Message::Menu),
        }
    }
}
