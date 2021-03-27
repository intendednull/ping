
run *ARGS:
	just run-node {{ ARGS }} &
	just run-web {{ ARGS }}

watch *ARGS:
	just watch-node {{ ARGS }} &
	just watch-web {{ ARGS }}
	

run-web *ARGS:
	trunk serve {{ ARGS }} ./web/index.html 

watch-web *ARGS:
	watchexec --exts rs,css,scss,html,js,toml -w common -w web -i dist -i pkg -r "just run-web {{ ARGS }}" 

run-node *ARGS:
	cargo build {{ ARGS }} --target=wasm32-wasi --bin node
	lunatic target/wasm32-wasi/debug/node.wasm

watch-node *ARGS:
	watchexec --exts rs,toml -w common -w node -r "just run-node {{ ARGS }}" 
