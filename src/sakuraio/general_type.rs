#[derive(Clone, Copy)]
pub enum SecurityLevel {
    Local,    //実行中の端末内からの通知。
    External, //実行中端末の外部からの通知。
}