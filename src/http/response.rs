use std::io::{Write, Result as IoResult};
use super::StatusCode;

pub struct Response {
    body: Option<String>,
    status_code: StatusCode,
}

impl Response {
    pub fn new(body: Option<String>, status_code: StatusCode) -> Self {
        Self {
            body,
            status_code,
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(value) => value,
            None => ""
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}