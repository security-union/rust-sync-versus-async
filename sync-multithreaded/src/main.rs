use std::{net::TcpListener, thread};

use types::{sync_handle_connection, ENDPOINT};

fn main() {
    let listener = TcpListener::bind(ENDPOINT).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || sync_handle_connection(stream));
    }
}
