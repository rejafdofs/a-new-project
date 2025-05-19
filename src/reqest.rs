use std::{clone, collections::HashSet};

pub struct Reqest {
    method: Method,
    charset: String,
    sender: String,
    sender_type: Option<HashSet<SenderType>>,
    security_level: SecurityLevel,
    security_origin: Option<String>,
    status: HashSet<Status>,
    shiori: Option<Shiori>,
}
impl Reqest {
    pub fn new(str: &str) -> Self {
        todo!()
    }
    pub fn get_method(&self) -> Method {
        self.method
    }
}
#[derive(Clone, Copy)]
pub enum Method {
    Get,    //レスポンスのValueヘッダでなにかしらを返すことを前提としているメソッド。
    Notify, //何も値を返さないことを前提としているメソッド。レスポンスのValueヘッダは無視される。主に何かしらのモードの制約で「喋れない」状態の時や、起動直後の状態通知イベントに使用される。同じID(イベント名)でGET/NOTIFYの両方とも使われる場合があるので注意。
}
#[derive(Clone, Copy)]
pub enum SenderType {
    Internal,    //アプリケーション内部からのイベント。
    External,    //アプリケーション外部からのイベント。
    Sakuraapi,   //Sakura APIによって起こされたイベント。
    Embed,       //\![embed]タグによって起こされたイベント。
    Raise,       //\![raise]タグによって起こされたイベント。
    Property,    //プロパティシステムの参照命令の結果起こされたイベント。
    Plugin,      //プラグインによって起こされたイベント。
    Sstp,        //SSTPによって起こされたイベント。
    Communicate, //コミュニケート仕様によって起こされたイベント。
}
#[derive(Clone, Copy)]
pub enum SecurityLevel {
    Local,    //実行中の端末内からの通知。
    External, //実行中端末の外部からの通知。
}
#[derive(Clone)]
pub enum Status {
    Talking,         //喋っている途中
    Choosing,        //選択肢表示中
    Minimizing,      //最小化中
    Induction,       //\![enter,inductionmode]中
    Passive,         //\![enter,passivemode]中
    Timecritical,    //タイムクリティカルセクション(\t)中
    Nouserbreak,     //\![enter,nouserbreak]中
    Online,          //ネットワーク通信中
    Opening(String), //入力ボックス等が開いている。複数種類が開いている場合は/区切りで列挙される。"dialog"はファイル選択・カラーピッカー等。例：opening(communicate/input/teach/dialog)
    Balloon(String), //バルーンが表示状態。キャラクターID=バルーンID の形式で列挙される。複数開いている場合は/区切りで列挙される。例：balloon(0=2/1=0) で、\0が大きいバルーン、\1が通常のバルーンを表示中の意。
}
#[derive(Clone)]
pub enum Shiori {
    ShioriEvent(ShioriEvent),
    ShioriResource(ShioriResource),
}
#[derive(Clone)]
pub enum ShioriEvent {}
#[derive(Clone, Copy)]
pub enum ShioriResource {}
