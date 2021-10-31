use crate::http::{Request};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
use crate::http::Handler;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address
        }
    }

    pub fn run(self, handler: impl Handler) {
        println!("Listening on: {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        // create an infinite loop to listen tcp address
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // allocate 1024 bytes space in the memory for our request
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // convert [0; 1024] to [u8] byte slice
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(_) => handler.handle_bad_request()
                            };

                            response.send(&mut stream);
                        }
                        Err(e) => println!("Failed read from connection: {}", e)
                    }
                }
                Err(e) => println!("Failed to receive request: {}", e)
            }
        }
    }
}