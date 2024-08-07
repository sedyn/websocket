use std::{
    io::{self, Read},
    net::{TcpListener, TcpStream},
};

use protocol::http::parse_http_request_message;

mod protocol;

fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in server.incoming() {
        println!("Incoming!");
        handle_connection(stream.unwrap()).unwrap();
    }
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()>{
    let mut buffer = vec![0; 4096];
    let size = stream.read(&mut buffer)?;

    let data = &buffer[..size];

    let message = parse_http_request_message(data);

    println!("{message:#?}");

    Ok(())
}
