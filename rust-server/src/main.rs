use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    sync::mpsc::{channel, Receiver, Sender},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not start TCP connection");
    let (msg_sender, msg_reciever) = channel();

    std::thread::spawn(|| server(msg_reciever));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
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

fn server(reciever: Receiver<Message>) {
    loop {
        let message = reciever.recv();
        match message {
            Ok(message) => match message {
                Message::Message => {
                    println!("Message Sent")
                }
                Message::NewConnection => {
                    println!("New Connection")
                }
                Message::ConnectionClosed => {
                    println!("Connection Closed")
                }
            },
            Err(_) => todo!(),
        }
    }
}

fn client(mut stream: TcpStream, sender: Sender<Message>) {
    sender
        .send(Message::NewConnection)
        .expect("Error sending message in channel");

    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    sender
                        .send(Message::ConnectionClosed)
                        .expect("Error sending message in channel");
                    break;
                }
                sender
                    .send(Message::Message)
                    .expect("Error sending message in channel");
            }
            Err(err) => {
                eprintln!("Error reading from client: {}", err);
                break;
            }
        }
    }
}

enum Message {
    Message,
    NewConnection,
    ConnectionClosed,
}
