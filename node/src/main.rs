mod client;
mod node;

use lunatic::net::TcpListener;

fn main() {
    env_logger::init();

    let node = wactor::spawn::<node::Node>();

    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).expect("Could not bind");
    loop {
        if let Ok(stream) = listener.accept() {
            let client = wactor::spawn::<client::Listener>();
            client
                .send(client::Config {
                    stream,
                    node: node.clone(),
                })
                .expect("Error spawning node")
        }
    }
}
