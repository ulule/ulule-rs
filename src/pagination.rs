// Most endpoints that return a list of resources use the same pagination system.
// The pagination is cursor-based and returns objects created before a timestamp
// in reverse chronological order.
// See https://developers.ulule.com

use serde::{Serialize, Deserialize};
use crate::params::Params;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub limit: u8,
    pub offset: Option<u8>,
    pub next: Option<String>
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
