.PHONY build:
build:
	cargo build --target wasm32-wasip2 --release
	wac plug --plug ./../signing/target/wasm32-wasip2/release/signing.wasm ./target/wasm32-wasip2/release/acme.wasm -o ./composed.wasm
