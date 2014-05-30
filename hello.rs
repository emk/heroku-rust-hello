extern crate rustful;
extern crate http;

use std::os::getenv;
use std::io::net::ip::Port;
use rustful::{Server, Router, Request, Response};
use http::method::Get;

// An example handler for "/".
fn hello(request: &Request, response: &mut Response) {
    match response.write("Hello from Rust!".as_bytes()) {
        Err(e) => println!("error: {}", e),
        _ => {}
    }
}

/// Look up our server port number in PORT, for
/// compatibility with Heroku.
fn get_server_port() -> Port {
    getenv("PORT")
        .and_then(|s| from_str::<Port>(s.as_slice()))
        .unwrap_or(8080)
}

/// Configure and run our server.
fn main() {
    let routes = [
        (Get, "/", hello)
    ];

    let server = Server::new(get_server_port(), Router::from_routes(routes));
    server.run();
}
