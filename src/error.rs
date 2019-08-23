use awc;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    /// An error reported by Ulule in the response body.
    Ulule(Vec<RequestError>),
    // A networking error communicating with the Ulule server.
    Http(awc::error::SendRequestError),
    // A set of errors that can occur during parsing payloads.
    Payload(awc::error::JsonPayloadError),
    // An error reading the response body.
    Io(std::io::Error),
}

/// RequestError is the error format returned by Ulule API.
/// [{"fieldNames":["tag_id"],"classification":"ValueError","message":"Must be an int64"}]
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestError {
    #[serde(alias = "fieldNames")]
    pub field_names: Vec<String>,
    pub message: String,
    pub classification: String,
}
