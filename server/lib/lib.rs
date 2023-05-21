pub extern crate http;

pub mod client;
pub mod cmd;
pub mod requests;
pub mod responses;

pub use client::ClientID;
pub use requests::Request;

pub trait HttpClient {
    type Error;

    fn send<R: Request>(&mut self, req: R) -> Result<R::Response, Self::Error>;
}

pub fn send<C: HttpClient, R: Request>(client: &mut C, req: R) -> Result<R::Response, C::Error> {
    client.send(req)
}
