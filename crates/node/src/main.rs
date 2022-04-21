mod connection;
mod node;

use lunatic::{net::TcpListener, process::Process, Mailbox};

#[lunatic::main]
fn main(_: Mailbox<()>) {
    env_logger::init();

    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).expect("Could not bind");
    let node = node::start();
    let mut connections: Vec<Process<()>> = vec![];
    loop {
        if let Ok((stream, _)) = listener.accept() {
            match connection::connect(node.clone(), stream) {
                Ok(con) => connections.push(con),
                Err(err) => log::error!("Unable to create connection: {:?}", err),
            }
        }
    }
}
