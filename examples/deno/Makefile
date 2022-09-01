build:
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen target/wasm32-unknown-unknown/release/*.wasm --target deno --out-dir build

deploy: build
	deployctl deploy --project=aws-region-nearby --include=build,index.ts index.ts --prod

.PHONY: build deploy
