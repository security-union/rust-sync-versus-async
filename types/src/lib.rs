use rand::Rng;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

pub const HTTP_RESPONSE_BODY: &str = include_str!("./hello.html");
pub const ENDPOINT: &str = "127.0.0.1:6667";

pub fn http_response() -> Vec<u8> {
    let status_line = "HTTP/1.1 200 OK";
    let contents = HTTP_RESPONSE_BODY;
    let length = contents.len();
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
        .as_bytes()
        .to_vec()
}

pub fn get_sleep_time() -> u64 {
    rand::thread_rng().gen_range(100..1000)
}

pub fn sync_handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // Simulate querying a database
    sleep(Duration::from_millis(get_sleep_time()));
    let response = http_response();

    stream.write_all(&response).unwrap();
}
