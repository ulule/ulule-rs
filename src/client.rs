use futures::future::Future;
use crate::error::Error;

use awc;
use serde;
use serde_json;

use actix_http::body::Body;

pub struct Client {
    client: awc::Client,
    host: String,
    ulule_version: String,
}

impl Client {
    pub fn new() -> Client {
        Client{
            client: awc::Client::default(),
            host: "https://api.ulule.com".to_string(),
            ulule_version: "2019-04-11".to_string()
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

    pub fn get<T: serde::de::DeserializeOwned>(&self, path: impl Into<String>, params: Option<impl Into<String>>)
        -> impl Future<Item=T, Error=Error> {
            let p = params.map_or("".to_string(), |par|(par.into()));
            let req = self.client
                      .get(self.host.to_owned()+&path.into()+&p)
                      .header("Ulule-Version", self.ulule_version.clone());

            self.send(req, Body::Empty)
        }

    fn send<T: serde::de::DeserializeOwned>(&self, req: awc::ClientRequest, body: Body)
        -> impl Future<Item=T, Error=Error> {
            req.send_body(body).map_err(|e| {Error::Http(e)})
                .and_then(|mut resp| {
                    resp.body().map(move |body_out| {
                        (resp, body_out)
                    }).map_err(|e| {Error::Payload(awc::error::JsonPayloadError::Payload(e))})
                }).and_then(|(_, body)| {
                    serde_json::from_slice(&body).map_err(|e| {Error::Payload(awc::error::JsonPayloadError::Deserialize(e))})
                })
        }
}
