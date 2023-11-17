use std::net::TcpListener;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not start TCP connection");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Connection Established: {}", stream.peer_addr().unwrap());

                let welcome_message = "Welcome to the server!\n";

                stream.write_all(welcome_message.as_bytes()).expect("Failed to send welcome message");
            },
            Err(err) => {
                eprintln!("Connection Attempt Failed: {}", err);
            },
        }
    }
}
