use serde::de::DeserializeOwned;

use ulule::error::RequestError;
use ulule::search;

pub struct Client {
    client: reqwest::Client,
    host: String,
    ulule_version: String,
}

impl Client {
    pub fn new() -> Client {
        Client {
            client: reqwest::Client::new(),
            host: "https://api.ulule.com".to_string(),
            ulule_version: "2019-04-11".to_string(),
        }
    }

    pub fn with_host(self, host: impl Into<String>) -> Client {
        let mut clt = self;
        clt.host = host.into();
        clt
    }

    pub fn with_ulule_version(self, version: impl Into<String>) -> Client {
        let mut clt = self;
        clt.ulule_version = version.into();
        clt
    }

    pub async fn get<T, S, U>(&self, path: S, params: Option<U>) -> Result<T, Error>
    where
        T: DeserializeOwned,
        S: Into<String>,
        U: Into<String>,
    {

        let p = params.map_or("".to_string(), |par| (par.into()));
        let url = self.host.to_owned() + &path.into() + &p;
        let res = self.client.get(&url)
            .header("Ulule-Version", &self.ulule_version)
            .send()
            .await
            .map_err(|e| Error::Http(e))?;

        if res.status().is_success() {
            let item: T =res.json().await.map_err(|e| Error::Http(e))?;
            return Ok(item)
        }

        let e: Vec<ulule::error::RequestError> = res.json().await.map_err(|e|{Error::Http(e)})?;
        Err(Error::Ulule(e))
    }
}

pub async fn search_projects(
    client: &Client,
    params: Option<impl Into<String>>,
) -> Result<search::Projects, Error> {
    client.get("/v1/search/projects", params).await
}

#[derive(Debug)]
pub enum Error {
    /// An error reported by Ulule in the response body.
    Ulule(Vec<RequestError>),
    /// An error reported by the reqwest client.
    Http(reqwest::Error)
}
