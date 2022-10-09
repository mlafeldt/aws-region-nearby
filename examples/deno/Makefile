build:
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen ../../target/wasm32-unknown-unknown/release/*.wasm --target deno --out-dir build

serve: build
	deno run -A --watch=build index.ts

deploy: build
	deployctl deploy --project=aws-region-nearby --include=build,index.ts index.ts --prod

.PHONY: build
