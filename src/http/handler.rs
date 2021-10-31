use super::{Request, Response, StatusCode};

pub trait Handler {
    fn handle_request(&self, request: &Request) -> Response;

    fn handle_bad_request(&self) -> Response {
        Response::new(None, StatusCode::BadRequest, None)
    }
}