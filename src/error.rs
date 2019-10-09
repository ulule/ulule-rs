use serde::{Deserialize, Serialize};

/// RequestError is the error format returned by Ulule API.
/// [{"fieldNames":["tag_id"],"classification":"ValueError","message":"Must be an int64"}]
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestError {
    #[serde(alias = "fieldNames")]
    pub field_names: Vec<String>,
    pub message: String,
    pub classification: String,
}
