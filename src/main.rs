use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

const IPPORT: &str = "127.0.0.1:3000";

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    println!("Request: {}", String::from_utf8_lossy(&buffer));

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 25\r\n\r\nI am the mighty response!";
    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(IPPORT)?;

    println!("Listening on {}", IPPORT);
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
