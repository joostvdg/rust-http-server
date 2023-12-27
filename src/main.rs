use server::Server;
use std::env;
use crate::website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let serverAddr = "127.0.0.1:8080".to_string();
    println!("Starting server on http://{} with public path: {}", serverAddr, public_path);
    let server = Server::new(serverAddr);
    server.run(WebsiteHandler::new(public_path));
}


