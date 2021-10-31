#![allow(dead_code)]

use server::Server;
use web_handler::WebHandler;

mod http;
mod server;
mod web_handler;

fn main() {
    let project_dir = env!("CARGO_MANIFEST_DIR");

    let server = Server::new("127.0.0.1:8000".to_string());

    let web_handler = WebHandler::new(format!("{}/public", project_dir));

    server.run(web_handler);
}
