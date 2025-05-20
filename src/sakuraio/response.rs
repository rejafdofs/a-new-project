use crate::define::GHOST_NAME;
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Response{
    startus_code:StartusCode,
    value:Option<String>,value_notify:Option<String>
}
impl Response{
    fn to_string(&self)->String{
format!("SHIORI/3.0 {} {}
Charset: UTF-8
Sender: {GHOST_NAME}
Value:{}")
    }
}#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum StartusCode {
    OK=200,NoContent=204,NotEnough=311,Advice=312,BadRequest=400,InternalServerError=500,IAmATeapot=418
}