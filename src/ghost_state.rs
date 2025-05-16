use std::sync::Mutex;
pub static GHOSTSTATE: Mutex<Ghoststate> = Mutex::new(Ghoststate::new());
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Ghoststate {}

impl Ghoststate {
    const fn new() -> Ghoststate {
        Ghoststate {}
    }
}
