use serde;

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(tag = "response_status")]
#[serde(rename_all = "snake_case")]
pub enum Response {
    #[serde(rename = "success")]
    SuccessStore,
    #[serde(rename = "success")]
    SuccessLoad {
        #[serde(rename = "requested_key")]
        key: String,
        #[serde(rename = "requested_hash")]
        hash: String,
    },
    KeyNotFound,
    Error,
}
