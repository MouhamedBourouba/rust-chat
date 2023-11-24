use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not start TCP connection");
    let (msg_sender, msg_reciever) = channel();

    std::thread::spawn(|| server(msg_reciever));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = Arc::new(stream);
                let cloned_sender = msg_sender.clone();
                std::thread::spawn(|| {
                    client(stream, cloned_sender);
                });
            }
            Err(err) => {
                eprintln!("Connection Attempt Failed: {}", err);
            }
        }
    }
}

fn server(reciever: Receiver<Event>) {
    let mut clinets = HashMap::<SocketAddr, Arc<TcpStream>>::new();

    loop {
        let message = reciever.recv();
        match message {
            Ok(message) => match message {
                Event::NewMessage { content, ip } => {
                    println!(
                        "Message recieved: {}:{}, {}",
                        ip.ip().to_string(),
                        ip.port().to_string(),
                        std::str::from_utf8(&content).unwrap()
                    );
                    for (_addr, stream) in &clinets {
                        stream.as_ref().write_all(&content).unwrap();
                    }
                }
                Event::NewConnection { stream, addr } => {
                    println!("New Connection");
                    clinets.insert(addr, stream);
                }
                Event::ConnectionClosed => {
                    println!("Connection Closed")
                }
            },
            Err(_) => todo!(),
        }
    }
}

fn client(stream: Arc<TcpStream>, sender: Sender<Event>) {
    let send = |message: Event| {
        sender
            .send(message)
            .expect("Error sending message in channel");
    };

    send(Event::NewConnection {
        stream: stream.clone(),
        addr: stream.peer_addr().unwrap(),
    });

    loop {
        let mut buffer = [0; 1024];

        match stream.as_ref().read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    send(Event::ConnectionClosed);
                    break;
                }
                send(Event::NewMessage {
                    content: Box::new(buffer),
                    ip: stream.peer_addr().unwrap(),
                });
            }
            Err(err) => {
                eprintln!("Error reading from client: {}", err);
                break;
            }
        }
    }
}

enum Event {
    NewMessage {
        content: Box<[u8]>,
        ip: SocketAddr,
    },
    NewConnection {
        stream: Arc<TcpStream>,
        addr: SocketAddr,
    },
    ConnectionClosed,
}
