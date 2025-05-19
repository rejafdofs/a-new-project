use crate::{
    reqest::Reqest,
    response::{self, Response},
};
use serde::{Deserialize, Serialize};
use std::{
    default, sync::{Arc, LazyLock, Mutex}, time::Duration
};
pub static GHOSTSTATE: LazyLock<Mutex<Status>> = LazyLock::new(|| {
    Mutex::new(Status::default())
});
#[derive(Clone,Default)]
pub struct Status {
    time: Duration,
    event_hook: Vec<Arc<dyn Fn(&Status, &Reqest) -> Option<Response> + Sync + Send>>, //次に來る特定のイベントについて豫め設定したresponseを返すときに使う
    ghost_status: GhostStatus,
}

impl Status {
    pub(crate) fn init(&mut self) {}
    pub(crate) fn save(&self) {}
    pub(crate) fn reqest(&mut self, reqest: Reqest) ->Option<Response> {
        todo!()
    }
    pub(crate) fn eval_hooks(&mut self, reqest: &Reqest) -> Option<Response> {
        if let Some((i, res)) = self
            .event_hook
            .iter()
            .enumerate()
            .find_map(|(i, hook)| (hook)(self, reqest).map(|res| (i, res)))
        {
            self.event_hook.remove(i);
            Some(res)
        } else {
            None
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,Default)]
pub struct GhostStatus {}
impl GhostStatus {
    const fn new() -> Self {
        GhostStatus {}
    }
}
