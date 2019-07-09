use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub limit: u8,
    pub next: String
}
