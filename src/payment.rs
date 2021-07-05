#[derive(Debug, Clone, Copy)]
pub enum Payment {
    Cash,
    Swish,
}

impl From<Payment> for String {
    fn from(p: Payment) -> String {
        String::from(match p {
            Payment::Swish => "Swish",
            Payment::Cash => "Cash",
        })
    }
}
