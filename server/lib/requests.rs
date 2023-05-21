use crate::http::{Method, Uri};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::responses;

pub trait Request: Sized {
    type Response: Serialize + DeserializeOwned;
    const PATH: &'static str;
    const METHOD: Method;

    fn into_http_request(self, authority_uri: String) -> http::Result<http::Request<Self>> {
        http::request::Builder::new()
            .uri(
                http::Uri::builder()
                    .authority(authority_uri)
                    .path_and_query(Self::PATH)
                    .scheme("https")
                    .build()?,
            )
            .method(Self::METHOD)
            .body(self)
    }
}

#[derive(Serialize)]
pub struct Connect;

impl Request for Connect {
    type Response = responses::Connect;
    const PATH: &'static str = "/client/connect";
    const METHOD: Method = Method::POST;
}

#[derive(Serialize)]
pub struct ListClients;

impl Request for ListClients {
    type Response = responses::ListClients;
    const PATH: &'static str = "/client/list";
    const METHOD: Method = Method::GET;
}
