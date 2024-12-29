use std::{
    io::{Read, Write},
    net::TcpStream,
};

use colored::Colorize;

pub enum HttpResponseCode {
    Success = 200,
    ServerError = 500,
}

impl ToString for HttpResponseCode {
    fn to_string(&self) -> String {
        match self {
            HttpResponseCode::Success => "200 OK".to_string(),
            HttpResponseCode::ServerError => "500 Internal Server Error".to_string(),
        }
    }
}

pub fn handle_request(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    let contents = String::from_utf8_lossy(&buffer);
    log_request(&contents);

    match contents {
        contents if contents.starts_with("GET") => handle_get(&stream),
        contents if contents.starts_with("POST") => handle_post(&stream),
        contents if contents.starts_with("PUT") => handle_put(&stream),
        contents if contents.starts_with("DELETE") => handle_delete(&stream),
        _ => write_response(
            &stream,
            get_response(
                HttpResponseCode::ServerError,
                "This HTTP method is not supported!",
            ),
        ),
    }?;

    Ok(())
}

fn handle_get(stream: &TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let response = get_response(HttpResponseCode::Success, "Hello from get!");
    write_response(stream, response)
}

fn handle_post(stream: &TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let response = get_response(HttpResponseCode::Success, "Hello from post!");
    write_response(stream, response)
}

fn handle_put(stream: &TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let response = get_response(HttpResponseCode::Success, "Hello from put!");
    write_response(stream, response)
}

fn handle_delete(stream: &TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let response = get_response(HttpResponseCode::Success, "Hello from delete!");
    write_response(stream, response)
}

fn write_response(
    mut stream: &TcpStream,
    response: String,
) -> Result<(), Box<dyn std::error::Error>> {
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn get_response(status: HttpResponseCode, content: &str) -> String {
    return format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status.to_string(),
        content.len(),
        content
    );
}

fn log_request(req: &str) {
    println!("\n{}: {}", "Request".cyan(), req.green())
}
