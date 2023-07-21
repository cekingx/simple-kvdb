use std::{net::{TcpListener, TcpStream}, io::{BufReader, prelude::*}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(stream);
    }
}

fn handle(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next();

    println!("lines: {:?}", request_line);
    stream.write_all("hello\n".as_bytes()).unwrap();
}