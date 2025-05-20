use crate::{
    interface::request,
    reqest::Reqest,
    response::{self, Response},
};
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, LazyLock, Mutex},
    time::Duration,
};
pub static STATUS: LazyLock<Mutex<Status>> = LazyLock::new(|| Mutex::new(Status::default()));
#[derive(Clone, Default)]
pub struct Status {
    time: Duration,
    ///起動からの時間
    path: String,
    event_hook: Vec<Arc<dyn Fn(&Status, &Reqest) -> Option<Response> + Sync + Send>>, //次に來る特定のイベントについて豫め設定したresponseを返すときに使う
    ghost_status: GhostStatus,
}

impl Status {
    pub(crate) fn init(&mut self, path: &str) {
        self.path = String::from(path);
        let _ = std::fs::read(path);
    }
    pub(crate) fn save(&self) {
        todo!()
    }
    pub(crate) fn reqest(&mut self, reqest: Reqest) -> Option<Response> {
        todo!()
    }
    fn add_hook(&mut self, hook: Arc<dyn Fn(&Status, &Reqest) -> Option<Response> + Sync + Send>) {
        self.event_hook.push(hook);
    }
    fn eval_hooks(&mut self, reqest: &Reqest) -> Option<Response> {
        self.event_hook
            .iter()
            .enumerate()
            .find_map(|(i, hook)| hook(self, reqest).map(|res| (i, res)))
            .map(|(i, res)| {
                self.event_hook.remove(i);
                res
            })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct GhostStatus {}
impl GhostStatus {}

