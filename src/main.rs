mod constants;
mod http;
mod request;
mod router;

use constants::{HttpMethod, HttpResponseCode};
use http::Http;

const ADDRESS: &str = "127.0.0.1:3000";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut http = Http::new();
    http.register_route(
        HttpMethod::GET,
        "/",
        Box::new(|stream| {
            let _ = Http::respond(
                stream,
                HttpResponseCode::Success,
                Some("This is the root route!"),
            );
        }),
    );

    http.register_route(
        HttpMethod::GET,
        "/hello",
        Box::new(|stream| {
            let _ = Http::respond(
                stream,
                HttpResponseCode::Success,
                Some("This is the hello route."),
            );
        }),
    );
    //http.register_route(HttpMethod::POST, "/post", Box::new(|stream| {}));
    //http.register_route(HttpMethod::PUT, "/", Box::new(|stream| {}));
    //http.register_route(HttpMethod::DELETE, "/del", Box::new(|stream| {}));
    http.listen(ADDRESS)?;
    Ok(())
}
