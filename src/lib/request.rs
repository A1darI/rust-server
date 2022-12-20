use serde;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "request_type")]
#[serde(rename_all = "snake_case")]
pub enum Request {
    Store { key: String, hash: String },
    Load { key: String },
}
