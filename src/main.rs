use std::{net::{TcpListener, TcpStream}, io::{BufReader, prelude::*, BufWriter}, str};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(stream);
    }
}

// fn handle(mut stream: TcpStream) {
//     let mut writer = stream.try_clone().unwrap();
//     let buf_reader = BufReader::new(&mut stream);

//     let response = format!(
//         "HTTP/1.1 200 OK\r\n\
//         Content-Length: 0\r\n\r\n\
//         "
//     );
//     for line in buf_reader.lines() {
//         println!("lines: {:?}", line);
//         writer.write_all(response.as_bytes()).unwrap();
//     }
// }

fn handle(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut clone = stream.try_clone().unwrap();
    let mut writer = BufWriter::new(&mut clone);
    stream.read(&mut buffer).unwrap();
    let x = str::from_utf8(&buffer).unwrap();
    println!("{}", x);
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Length: 0\r\n\r\n\
        "
    );
    writer.write_all(response.as_bytes()).unwrap();
}