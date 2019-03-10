use std::io::prelude::*;
use std::net::TcpListener;
use std::{thread, time};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let status = "HTTP/1.1 200 OK\r\n";
    let header = "Content-Type: text/html; charset=UTF-8\r\n\r\n";
    let response = "💩";
    let one_sec = time::Duration::from_millis(200);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        stream.write(status.as_bytes()).unwrap();
        stream.write(header.as_bytes()).unwrap();

        loop {
            stream.write(response.as_bytes()).unwrap();

            // Sleep for a while every loop
            thread::sleep(one_sec);
        }
    }
}
