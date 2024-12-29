mod http;

use std::net::TcpListener;

use http::handle_request;

const IPPORT: &str = "127.0.0.1:3000";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(IPPORT)?;

    println!("Listening on {}", IPPORT);
    for stream in listener.incoming() {
        handle_request(stream?)?;
    }
    Ok(())
}
