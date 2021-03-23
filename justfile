
run:
        just run-node &
        trunk serve ./web/index.html 

run-node:
        cargo build --target=wasm32-wasi --bin node
        lunatic target/wasm32-wasi/debug/node.wasm
    
