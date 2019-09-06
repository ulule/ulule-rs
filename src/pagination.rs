// Most endpoints that return a list of resources use the same pagination system.
// The pagination is cursor-based and returns objects created before a timestamp
// in reverse chronological order.
// See https://developers.ulule.com

use crate::params::Params;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub limit: u64,
    pub offset: Option<u64>,
    pub next: Option<String>,
}

impl Page {
    pub fn has_next(&self) -> bool {
        match self.next {
            None => false,
            Some(_) => true,
        }
    }

    pub fn get_next(self) -> Option<Params> {
        match self.next {
            None => None,
            Some(s) => Some(Params::from_str(&s).unwrap()),
        }
    }
}
