init:
	cd www && npm install

build:
	wasm-pack build

run:
	cd www && npm run start

dist:
	wasm-pack build && cd www && ./node_modules/.bin/webpack

ci:
	cd www && npm ci
