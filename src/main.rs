use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use postgres::{Client, NoTls};

pub mod threadpool;

use crate::threadpool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if http_request.is_empty() {
        return;
    }

    let request_line = &http_request[0];

let (status_line, content) = match request_line.as_str() {
    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html".to_owned()),
    "GET /songs HTTP/1.1" => {
        // Code to get all songs
        let mut client = Client::connect("host=localhost dbname=SEGURIDAD user=api password=F2wYzVzee6RNmpsBWqbxLWAMvywqyYfDsZawAwnfYhvEjoJL3YWCBTDPeyrLiygv4YXBnpM7KeJjaGyVHvyzAKWxfPK9uYueBayc", NoTls)
            .expect("Failed to connect to database");
        let mut result: String = String::new();
        for row in client.query("SELECT title, artist FROM songs", &[]).expect("Failed to select songs") {
            let name: &str = row.get(0);
            let artist: &str = row.get(1);
            result.push_str(&format!("{} - {}\n", name, artist));
        }
        ("HTTP/1.1 200 OK", result)
    }
    _ if request_line.starts_with("GET /songs/") => {
        let parts: Vec<&str> = request_line.split(' ').collect();
        let path = parts[1];

        let uuid_str = path.trim_start_matches("/songs/").trim_end_matches(" HTTP/1.1");
        
        let mut client = Client::connect("host=localhost dbname=SEGURIDAD user=api password=F2wYzVzee6RNmpsBWqbxLWAMvywqyYfDsZawAwnfYhvEjoJL3YWCBTDPeyrLiygv4YXBnpM7KeJjaGyVHvyzAKWxfPK9uYueBayc", NoTls)
            .expect("Failed to connect to database");

        if let Ok(row) = client.query_one("SELECT title, artist FROM songs WHERE id = $1::uuid", &[&uuid_str]) {
            let title: &str = row.get(0);
            let artist: &str = row.get(1);
            let result = format!("{} - {}", title, artist);
            ("HTTP/1.1 200 OK", result)
        } else {
            ("HTTP/1.1 404 NOT FOUND", "Song not found or invalid UUID format.".to_owned())
        }
    }
    _ => ("HTTP/1.1 404 NOT FOUND", "404.html".to_owned()),
};

    let length = content.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\nConnection: close\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
