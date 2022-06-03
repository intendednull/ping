

use lunatic::{
    net::TcpStream,
    process::{self, Process},
    LunaticError, Mailbox,
};

use tungstenite::{protocol::Role, server, WebSocket};

use common::transport as t;

use crate::node;

#[derive(Debug)]
pub enum ConnectionError {
    Lunatic(LunaticError),
}

pub fn connect(
    node: Process<node::Msg>,
    stream: TcpStream,
) -> Result<Process<()>, ConnectionError> {
    process::spawn_with((node, stream), |(node, stream), _mailbox: Mailbox<()>| {
        let mut ws = server::accept(stream.clone()).expect("Error creating WS");

        // Spawn a responder to handle responsed from node. Needed because this listener will
        // always be blocking to read messages.
        let sender = process::spawn_with(stream, |stream, mailbox: Mailbox<t::Output>| {
            let mut ws = Some(WebSocket::from_raw_socket(stream, Role::Server, None));
            while let Ok(msg) = mailbox.receive() {
                if let Ok(data) = t::pack(&msg) {
                    ws.as_mut()
                        .expect("not initialized")
                        .write_message(data.into())
                        .ok();
                }
            }
        })
        .expect("Could not create sender");

        loop {
            match ws.read_message() {
                Ok(tungstenite::Message::Binary(msg)) => {
                    if let Ok(msg) = t::unpack(&msg) {
                        log::info!("Received: {:?}", msg);

                        node.send(node::Msg::Request(sender.clone(), msg));
                    }
                }
                Err(_e) => break,
                _ => {}
            }
        }
    })
    .map_err(ConnectionError::Lunatic)
}
