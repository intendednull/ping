use lunatic::net::{TcpListener, TcpStream};
use tungstenite::server;
use wactor::*;

use common::Message;

struct Node;
impl Actor for Node {
    type Input = TcpStream;
    type Output = ();

    fn create() -> Self {
        Self
    }

    fn handle(&mut self, stream: Self::Input, _link: &Link<Self>) {
        let mut ws = server::accept(stream).expect("Error creating WS");
        loop {
            let msg = ws.read_message();
            match msg {
                Ok(tungstenite::Message::Binary(msg)) => {
                    let msg =
                        bincode::deserialize::<Message>(&msg).expect("Error deserializing message");

                    let response = match msg {
                        Message::Ping => Message::Pong,
                        Message::Pong => Message::Ping,
                    };

                    let buf = bincode::serialize(&response).expect("Error serializing response");
                    ws.write_message(buf.into())
                        .expect("Error sending response");
                }
                Err(_) => break,
                _ => {}
            }
        }
    }
}

fn main() {
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).expect("Could not bind");
    loop {
        if let Ok(stream) = listener.accept() {
            wactor::spawn::<Node>()
                .send(stream)
                .expect("Error spawning node")
        }
    }
}
