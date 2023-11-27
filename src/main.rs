mod handle_request;
mod http;
mod thread;
mod util;

use anyhow::Error;
use std::net::TcpListener;
use std::thread::{self as stdthread};

#[macro_use]
extern crate slog;

fn main() -> Result<(), Error> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    println!("Main ThreadID is: {:?}", stdthread::current().id());

    let mut pool = thread::ConcTcpStreamPool::new(10);

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("receive a connect");
                pool.dispatch(stream)?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    pool.stop()?;
    return Ok(());
}
