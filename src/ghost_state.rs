use serde::{Deserialize, Serialize};
use std::{sync::Mutex, time::Duration};
use crate::reqest::Reqest;
pub static GHOSTSTATE: Mutex<Ghoststate> = Mutex::new(Ghoststate::new());
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct Ghoststate {
    time: Duration,
}

impl Ghoststate {
    const fn new() -> Ghoststate {
        Ghoststate {
            time: Duration::new(0, 0),
        }
    }
    pub(crate) fn init(&mut self) {}
    pub(crate) fn save(&self) {}
    pub(crate) fn reqest(&self,reqest:Reqest)->Self {
        todo!()
    }
}
