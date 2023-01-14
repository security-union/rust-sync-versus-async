use std::net::TcpListener;

use types::{sync_handle_connection, ENDPOINT};

fn main() {
    let listener = TcpListener::bind(ENDPOINT).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        sync_handle_connection(stream);
    }
}
