use std::{net::{TcpListener, TcpStream}, io::{BufReader, prelude::*}, collections::HashMap};

type Database = HashMap<String, String>;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut db = HashMap::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(&mut db, stream);
    }
}

fn handle(db: &mut Database, mut stream: TcpStream) {
    let mut writer = stream.try_clone().unwrap();
    let buf_reader = BufReader::new(&mut stream);

    for line in buf_reader.lines() {
        println!("lines: {:?}", line);
        let result = line.unwrap();
        let cloned = result.clone();
        let parsed: Vec<&str> = cloned.split(" ").collect();

        if parsed[0] == "set" {
            db.insert(parsed[1].to_string(), parsed[2].to_string());
            let response = format!("Ok\n");
            writer.write_all(response.as_bytes()).unwrap();
        }

        if parsed[0] == "get" {
            let result = db.get(parsed[1]);
            match result {
                Some(value) => {
                    let response = format!("{}\n", value.to_string());
                    writer.write_all(response.as_bytes()).unwrap();
                }
                None => {
                    let response = format!("\n");
                    writer.write_all(response.as_bytes()).unwrap();
                }
            }
        }

    }
}