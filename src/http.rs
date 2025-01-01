use std::error::Error;
use std::net::TcpListener;

use std::{
    io::{Read, Write},
    net::TcpStream,
};

use colored::Colorize;

use crate::constants::{HttpMethod, HttpResponseCode};
use crate::request::Request;
use crate::router::Router;

pub struct Http {
    router: Router,
}

impl Http {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn listen(&mut self, address: &str) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(address)?;

        println!("Listening on {}", address);
        for stream in listener.incoming() {
            self.handle_request(stream?)?;
        }

        Ok(())
    }

    pub fn register_route(
        &mut self,
        method: HttpMethod,
        path: &str,
        callback: Box<dyn Fn(&TcpStream, Request) -> ()>,
    ) {
        match self.router.add_route(method, &path, callback) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        }
    }

    fn handle_request(&mut self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer)?;

        let contents = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        self.log_request(&contents);

        let request = Request::new(contents);

        let request: Request = match request {
            Ok(value) => value,
            Err(_) => {
                let _ = Self::respond(
                    &stream,
                    HttpResponseCode::ServerError,
                    Some("There was an error parsing your request!"),
                );
                return Ok(());
            }
        };

        let route = match request.method {
            HttpMethod::GET => self.router.get_route(HttpMethod::GET, &request.path),
            HttpMethod::POST => self.router.get_route(HttpMethod::POST, &request.path),
            HttpMethod::PUT => self.router.get_route(HttpMethod::PUT, &request.path),
            HttpMethod::DELETE => self.router.get_route(HttpMethod::DELETE, &request.path),
            HttpMethod::NONE => None,
        };
        let _ = match route {
            Some(r) => {
                (r.callback)(&stream, request);
                Ok(())
            }
            None => Self::respond(&stream, HttpResponseCode::NotFound, None),
        };

        Ok(())
    }

    pub fn respond(
        stream: &TcpStream,
        status: HttpResponseCode,
        content: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        Self::write_response(stream, Self::get_response(status, content))
    }

    fn write_response(
        mut stream: &TcpStream,
        response: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        stream.write(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }

    fn get_response(status: HttpResponseCode, content: Option<&str>) -> String {
        let content = match content {
            Some(value) => format!(
                "\r\nContent-Length: {length}\r\n\r\n{value}",
                length = value.len(),
                value = value
            ),
            None => format!("\r\nContent-Length: 0"),
        };
        return format!(
            "HTTP/1.1 {status}{content}",
            status = status,
            content = content
        );
    }

    fn log_request(&self, req: &str) {
        println!("\n{}: {}", "Request".cyan(), req.green())
    }
}
