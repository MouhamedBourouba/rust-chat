use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not start TCP connection");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(err) => {
                eprintln!("Connection Attempt Failed: {}", err);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("New client, ip: {}", stream.peer_addr().unwrap());
    loop {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Connection closed by client");
                    break;
                }

                let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received message: {}", received_message);
            }
            Err(err) => {
                eprintln!("Error reading from client: {}", err);
                break;
            }
        }
    }
}
