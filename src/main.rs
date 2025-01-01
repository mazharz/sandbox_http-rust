mod constants;
mod http;
mod request;
mod router;

use constants::{HttpMethod, HttpResponseCode};
use http::Http;
use serde_json::Value;

const ADDRESS: &str = "127.0.0.1:3000";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut http = Http::new();
    http.register_route(
        HttpMethod::GET,
        "/",
        Box::new(|stream, _request| {
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
        Box::new(|stream, _request| {
            let _ = Http::respond(
                stream,
                HttpResponseCode::Success,
                Some("This is the hello route."),
            );
        }),
    );
    http.register_route(
        HttpMethod::POST,
        "/",
        Box::new(|stream, request| {
            let mut a = String::from("");

            let data = request.data;

            if let Some(boxed_data) = data {
                if let Some(json_value) = boxed_data.downcast_ref::<Value>() {
                    if let Some(aa) = json_value.get("a").and_then(|v| v.as_str()) {
                        a = aa.to_owned();
                    }
                }
            }

            let _ = Http::respond(
                stream,
                HttpResponseCode::Success,
                Some(&format!("value of a was: {}", a)),
            );
        }),
    );
    //http.register_route(HttpMethod::PUT, "/", Box::new(|stream| {}));
    //http.register_route(HttpMethod::DELETE, "/del", Box::new(|stream| {}));
    http.listen(ADDRESS)?;
    Ok(())
}
