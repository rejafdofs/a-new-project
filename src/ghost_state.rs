use serde::{Deserialize, Serialize};
use std::sync::Mutex;
pub static GHOSTSTATE: Mutex<Ghoststate> = Mutex::new(Ghoststate::new());
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct Ghoststate {}

impl Ghoststate {
    const fn new() -> Ghoststate {
        Ghoststate {}
    }
    fn init(&mut self){

    }
    fn save(&self){
        
    }
}
