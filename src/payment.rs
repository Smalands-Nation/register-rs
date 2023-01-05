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
            _ => Err("Invalid Payment Method".into()),
        }
    }
}

use {iced::widget::image::Image, iced::Element, iced_native::image::Handle};
impl<'a, M> From<Payment> for Element<'a, M>
where
    M: Clone + 'a,
{
    fn from(p: Payment) -> Self {
        Image::new(Handle::from_memory(match p {
            Payment::Swish => include_bytes!("../resources/swish.png").to_vec(),
            Payment::Paypal => include_bytes!("../resources/paypal.png").to_vec(),
            _ => unreachable!("Payment to Image"),
        }))
        .into()
    }
}
