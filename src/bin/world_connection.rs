
use std::net::TcpListener;

fn main() {
    let port = 8080;

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    println!("server started on port : {}", port );

    for stream in listener.incoming() {
        
        let stream = stream.unwrap();
    }
}
