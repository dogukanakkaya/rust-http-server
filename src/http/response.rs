use std::io::{Write, Result as IoResult};
use super::{StatusCode};

pub struct Response<'a> {
    body: Option<String>,
    status_code: StatusCode,
    headers: Option<Vec<&'a str>>,
}

impl<'a> Response<'a> {
    pub fn new(body: Option<String>, status_code: StatusCode, headers: Option<Vec<&'a str>>) -> Self {
        Self {
            body,
            status_code,
            headers,
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(value) => value,
            None => ""
        };
        let headers = match &self.headers {
            Some(h) => h.join("\r\n"),
            None => String::new()
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            headers,
            body
        )
    }
}