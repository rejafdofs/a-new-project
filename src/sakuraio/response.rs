
use crate::define::GHOST_NAME;
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Response {
    startus_code: StartusCode,
    value: Option<String>,
    value_notify: Option<String>,
}
impl Response {
    fn to_string(&self) -> String {
        format!(
            "SHIORI/3.0 {} {}
Charset: UTF-8
Sender: {GHOST_NAME}
{}",
            (self.startus_code as u8).to_string(),
            self.startus_code.to_string(),
            match &self.value {
                Some(t) => format!("Value:{}", t),
                None => "".to_string(),
            }
        )
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum StartusCode {
    OK = 200,
    NoContent = 204,
    NotEnough = 311,
    Advice = 312,
    BadRequest = 400,
    InternalServerError = 500,
    IAmATeapot = 418,
}
impl StartusCode {
    fn to_string(&self) -> String {
        match self {
            StartusCode::OK => stringify!(OK).to_string(),
            StartusCode::NoContent => stringify!(No Content).to_string(),
            StartusCode::NotEnough=>stringify!(Not Enough).to_string(),
            StartusCode::Advice => stringify!(Advice).to_string(),
            StartusCode::BadRequest => stringify!(Bad Request).to_string(),
            StartusCode::InternalServerError => stringify!(Internal Server Error).to_string(),
            StartusCode::IAmATeapot => stringify!(I Am A Teapot).to_string(),
        }
    }
}
