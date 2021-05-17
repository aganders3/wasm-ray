wasm-init:
	cd www && npm install

wasm:
	wasm-pack build

wasm-run:
	cd www && npm run start

wasm-dist:
	wasm-pack build && cd www && ./node_modules/.bin/webpack

debug:
	cargo build

bin:
	cargo build --release