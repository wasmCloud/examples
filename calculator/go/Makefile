NAME = calculator

.PHONY: all build check clean doc test update

all: build

start:
	RUST_LOG=debug wasmcloud --allowed-insecure localhost:5000 -m manifest.yaml

build:
	mkdir -p build
	tinygo build -o build/$(NAME).wasm -target wasm -no-debug main.go
	wash claims sign build/$(NAME).wasm -q -h -l --name $(NAME) --ver 0.1.0 --rev 0
	wash drain all
	wash reg push localhost:5000/calc:0.1.0 build/calculator_s.wasm --insecure
