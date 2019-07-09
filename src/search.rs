use futures::future::Future;
use serde::{Serialize, Deserialize};

use crate::client::Client;
use crate::params;
use crate::pagination::Page;
use crate::i18n::I18n;
use crate::error::Error;

// Project is a search resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    // Link to the project page on the Ulule website
    pub absolute_url: String,
    // Amount raised in project currency
    pub amount_raised: u64,
    // Two-letter ISO code of the country
    pub country: String,
    // Two-letter ISO code of the currency
    pub currency: String,
    // Date at which the funding campaign ends, with RFC 3339 format
    pub date_end: String,
    // Date at which the funding campaign starts, with RFC 3339 format
    pub date_start: String,
    // True if the funding campaign is finished
    pub finished: bool,
    // Goal in the project currency if type is project, or number of pre-orders if type is presale.
    pub goal: u64,
    // Unique id of the project
    pub id: u64,
    // Main language of the project
    pub lang: String,
    // Name of the project
    pub name: I18n,
    // Number of products sold
    pub nb_products_sold: u64,
    // Unique slug of the project
    pub slug: String,
    // Subtitle of the project
    pub subtitle: I18n,
    // Type of the project (presale or project)
    #[serde(alias = "type")]
    pub kind: String,
}

// Projects is a paginated list of search project resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Projects {
    pub projects: Vec<Project>,
    pub meta: Page
}

pub type Params = params::Params;

impl Params {
    pub fn new_limit(self, l: u64) -> Params {
        self.limit(l)
    }
}

impl From<Params> for std::string::String {
    fn from(p: Params) -> Self {
        p.to_string()
    }
}



pub fn projects(client: &Client, params: Option<Params>) -> impl Future<Item=Projects, Error=Error> {
    client.get("/v1/search/projects", params)
}
