TARGET = ./rust/target/wasm32-unknown-unknown
DEBUG = $(TARGET)/debug
RELEASE = $(TARGET)/release

build-all: build-rust build-as build-go

# Rust rules
build-rust:
	cd ./rust; cargo build	
	wash claims sign $(DEBUG)/image_host.wasm -c wasmcloud:httpserver -c wasmcloud:blobstore --name "Image Host (Rust)" --ver 0.1.0 --rev 0

run-rust: build-rust
	IMAGE_HOST_WASM=./rust/target/wasm32-unknown-unknown/debug/image_host_s.wasm \
	IMAGE_HOST_PKEY=$(shell wash claims inspect ./rust/target/wasm32-unknown-unknown/debug/image_host_s.wasm -o json | jq ".module" | tr -d "\"") \
	RUST_LOG=info \
	wasmcloud --manifest manifest.yaml

# Assemblyscript Rules
build-as:
	cd ./as; npm run build
	wash claims sign ./as/build/imagehost-as.wasm -d ./as/build/imagehost-as_s.wasm -c wasmcloud:httpserver -c wasmcloud:blobstore --name "Image Host (AS)" --ver 0.1.0 --rev 0

run-as: build-as
	IMAGE_HOST_WASM=./as/build/imagehost-as_s.wasm \
	IMAGE_HOST_PKEY=$(shell wash claims inspect ./as/build/imagehost-as_s.wasm -o json | jq ".module" | tr -d "\"") \
	RUST_LOG=info \
	wasmcloud --manifest manifest.yaml

# Tinygo Rules
build-go:
	cd ./tinygo; tinygo build -o build/tinygo.wasm -target wasm -no-debug main.go
	wash claims sign ./tinygo/build/tinygo.wasm -d ./tinygo/build/imagehost-go_s.wasm -c wasmcloud:httpserver -c wasmcloud:blobstore --name "Image Host (Go)" --ver 0.1.0 --rev 0

run-go: build-go
	IMAGE_HOST_WASM=./tinygo/build/imagehost-go_s.wasm \
	IMAGE_HOST_PKEY=$(shell wash claims inspect ./tinygo/build/imagehost-go_s.wasm -o json | jq ".module" | tr -d "\"") \
	RUST_LOG=info \
	wasmcloud --manifest manifest.yaml
