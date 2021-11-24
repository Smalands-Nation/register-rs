#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Payment {
    Cash,
    Swish,
    Paypal,
}

impl Default for Payment {
    fn default() -> Self {
        Self::Swish
    }
}

impl From<Payment> for String {
    fn from(p: Payment) -> String {
        String::from(match p {
            Payment::Swish => "Swish",
            Payment::Cash => "Cash",
            Payment::Paypal => "PayPal",
        })
    }
}

impl TryFrom<String> for Payment {
    type Error = crate::error::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "cash" => Ok(Self::Cash),
            "swish" => Ok(Self::Swish),
            "paypal" => Ok(Self::Paypal),
            _ => Err("Invalid Payment Method")?,
        }
    }
}
