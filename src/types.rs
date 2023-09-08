use serde::{Deserialize, Serialize};

pub type Products = Vec<Product>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductResponse {
    pub code: String,
    pub message: Option<String>,
    pub message_detail: Option<String>,
    pub data: Products,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub s: String,
    pub st: String,
    pub b: String,
    pub q: String,
    pub ba: String,
    pub qa: String,
    pub i: String,
    pub ts: String,
    pub an: String,
    pub qn: String,
    pub o: String,
    pub h: String,
    pub l: String,
    pub c: String,
    pub v: String,
    pub qv: String,
    pub y: i32,
    #[serde(rename = "as")]
    pub avs: f64,
    pub pm: String,
    pub pn: String,
    pub cs: i64,
    pub tags: Vec<String>,
    pub pom: bool,
    pub pomt: Option<i64>,
    pub lc: bool,
    pub g: bool,
    pub sd: bool,
    pub r: bool,
    pub hd: bool,
    pub rb: bool,
    pub ks: bool,
    pub etf: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocketData {
    pub stream: String,
    pub data: Vec<MiniTicker>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MiniTicker {
    pub e: Option<String>,
    #[serde(rename = "E")]
    pub ev_time: Option<u64>,
    pub s: String,
    pub c: String,
    pub o: String,
    pub h: String,
    pub l: String,
    pub v: String,
    pub q: String,
}
