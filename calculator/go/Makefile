NAME = calculator

.PHONY: all build check clean doc test update

all: build

start:
	RUST_LOG=debug wasmcloud -m manifest.yaml

build:
	mkdir -p build
	tinygo build -o build/$(NAME).wasm -target wasm -no-debug main.go
	wash claims sign build/$(NAME).wasm -q -h -l --name $(NAME) --ver 0.1.0 --rev 0 --destination calculator_s.wasm
