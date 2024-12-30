mod constants;
mod http;
mod request;
mod router;

use constants::HttpMethod;
use http::Http;

const ADDRESS: &str = "127.0.0.1:3000";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut http = Http::new();
    http.register_route(HttpMethod::GET, "/");
    http.register_route(HttpMethod::POST, "/post");
    http.register_route(HttpMethod::PUT, "/");
    http.register_route(HttpMethod::DELETE, "/del");
    http.listen(ADDRESS)?;
    Ok(())
}
