use crate::State;
use tide::{Request, Response};

pub(super) async fn list(state: Request<State>) -> tide::Result {
    Ok("session list".into())
}

pub(super) async fn create(state: Request<State>) -> tide::Result {
    Ok("session created".into())
}

pub(super) async fn join(state: Request<State>) -> tide::Result {
    Ok("session joined".into())
}
