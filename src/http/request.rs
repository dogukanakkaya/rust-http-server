use std::convert::TryFrom;
use std::error::Error;
use super::{Method, QueryString};
use std::fmt::{
    Display,
    Debug,
    Formatter,
};
use std::str;

#[derive(Debug)]
pub struct Request<'buf> {
    method: Method,
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
}

impl <'buf> Request<'buf> {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &'buf str {
        self.path
    }

    pub fn query_string(&self) -> Option<&QueryString<'buf>> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = RequestError;

    fn try_from(buffer: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buffer).or(Err(RequestError::InvalidEncoding))?;

        let (method, request) = get_next_word(request).ok_or(RequestError::InvalidMethod)?;
        let (mut path, request) = get_next_word(request).ok_or(RequestError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(RequestError::InvalidProtocol)?;

        let method: Method = method.parse().or(Err(RequestError::InvalidMethod))?;

        if protocol != "HTTP/1.1" {
            return Err(RequestError::InvalidProtocol);
        }

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));

            path = &path[..i];
        }

        Ok(Self {
            method,
            path,
            query_string,
        })
    }
}

pub enum RequestError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl RequestError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Error for RequestError {}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for RequestError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // enumerate returns us another iterator to that yields a tuple of (index, item)
    for (i, char) in request.chars().enumerate() {
        if char == ' ' || char == '\r' {
            // if the char is space, return the chars until the space
            // and chars after space with (exclude space with i + 1)
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    // return none if no word left
    None
}