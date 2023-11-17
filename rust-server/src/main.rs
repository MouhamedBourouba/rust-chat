use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not start TCP connection");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection Established: {}", stream.peer_addr().unwrap());
                println!("Message recieved");
            }
            Err(err) => {
                eprintln!("Connection Attempt Failed: {}", err);
            }
        }
    }
}
