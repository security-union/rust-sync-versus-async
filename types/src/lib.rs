use std::{
    io::{prelude::*, BufReader},
    net::TcpStream, thread::sleep, time::Duration
};
use lazy_static::lazy_static;
use tokio::time::sleep as tokio_sleep;

pub const SLEEP_TIME_MS: u64 = 100;
pub const HTTP_RESPONSE_BODY: &str = include_str!("./hello.html");
pub const ENDPOINT: &str = "127.0.0.1:6667";

lazy_static! {
    pub static ref CHALLENGE: String = std::env::var("CHALLENGE").unwrap_or_else(|_| "STATIC_WEBSITE".to_string());
}

pub fn http_response() -> Vec<u8> {
    let status_line = "HTTP/1.1 200 OK";
    let contents = HTTP_RESPONSE_BODY;
    let length = contents.len();
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
        .as_bytes()
        .to_vec()
}

pub fn sync_handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    sync_run_challenge();
    let response = http_response();

    stream.write_all(&response).unwrap();
}


pub fn permute_string(s: &str, l: usize, r: usize, permutations: &mut Vec<String>) {
    if l == r {
        permutations.push(s.to_string());
    } else {
        for i in l..=r {
            let mut chars: Vec<char> = s.chars().collect();
            chars.swap(l, i);
            permute_string(&chars.into_iter().collect::<String>(), l + 1, r, permutations);
        }
    }
}

pub fn sync_run_challenge() {
    match CHALLENGE.as_str() {
        "STATIC_WEBSITE" => {
            sleep(Duration::from_millis(SLEEP_TIME_MS));
        }
        _ => permute()
    }
}

pub async fn async_run_challenge() {
    match CHALLENGE.as_str() {
        "STATIC_WEBSITE" => {
            tokio_sleep(Duration::from_millis(SLEEP_TIME_MS)).await;
        }
        _ => permute()
    }
}

pub fn permute() {
    let mut permutations = Vec::new();
    let s = "fjlka22";
    let n = s.len();
    permute_string(s, 0, n-1, &mut permutations);
}