use crate::http::{Request, Response, StatusCode, Handler, Method};
use std::fs;

pub struct WebHandler {
    public_path: String,
}

impl WebHandler {
    pub fn new(public_path: String) -> Self {
        Self {
            public_path
        }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack on path: {}", file_path);
                    None
                }
            },
            Err(_) => None
        }
    }
}

impl Handler for WebHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => {
                    let headers = Vec::from([
                        "Accept: application/html",
                        "Age: 132423"
                    ]);

                    Response::new(self.read_file("index.html"), StatusCode::Ok, Some(headers))
                },
                "/about" => Response::new(self.read_file("about.html"), StatusCode::Ok, None),
                path => match self.read_file(path) {
                    Some(content) => Response::new(Some(content), StatusCode::Ok, None),
                    None => Response::new(None, StatusCode::NotFound, None)
                }
            },
            _ => Response::new(None, StatusCode::BadRequest, None)
        }
    }
}