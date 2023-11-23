use std::{
    io::{self, Read},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                handle_tcpstream(&mut _stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_tcpstream(ts: &mut TcpStream) {
    let mut buffer = Vec::new();
    // read the whole file
    let _ = ts.read(&mut buffer);
    println!("buf = {:?}", buffer);
}
