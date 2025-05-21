use crate::sakuraio::{request::Request, response::Response};
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, LazyLock, Mutex},
    time::Duration,
};
pub static STATUS: LazyLock<Mutex<Status>> = LazyLock::new(|| Mutex::new(Status::default()));
#[derive(Clone, Default)]
pub struct Status {
    time: Duration, //起動からの時間
    path: String,
    event_ankers: Vec<Arc<dyn Fn(&Status, &Request) -> Option<Response> + Sync + Send>>, //次に來る特定のイベントについて豫め設定したresponseを返すときに使う
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
    pub(crate) fn reqest(&mut self, reqest: Request) -> Option<Response> {
        todo!()
    }
    fn add_anker<F>(&mut self, hook: F)
    where
        F: Fn(&Status, &Request) -> Option<Response> + Send + Sync + 'static,
    {
        self.event_ankers.push(Arc::new(hook));
    }
    fn eval_ankers(&mut self, reqest: &Request) -> Option<Response> {
        self.event_ankers
            .iter()
            .enumerate()
            .find_map(|(i, anker)| anker(self, reqest).map(|res| (i, res)))
            .map(|(i, res)| {
                self.event_ankers.remove(i);
                res
            })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct GhostStatus {}
impl GhostStatus {}
