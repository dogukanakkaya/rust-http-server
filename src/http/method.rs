use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "PATCH" => Ok(Method::PATCH),
            "DELETE" => Ok(Method::DELETE),
            _ => Err(String::from("Error"))
        }
    }
}