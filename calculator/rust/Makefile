COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
TARGET = target/wasm32-unknown-unknown
DEBUG = $(TARGET)/debug
RELEASE = $(TARGET)/release
KEYDIR ?= .keys
VERSION = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[].version')

.PHONY: all build check clean doc test update

all: release

build:
	@$(CARGO) build	
	wash claims sign $(DEBUG)/calculator.wasm -c wasmcloud:httpserver --name "calculator" --ver $(VERSION) --rev 0

start:
	RUST_LOG=debug wasmcloud --allowed-insecure localhost:5000 -m manifest.yaml

check:
	@$(CARGO) check

clean:
	@$(CARGO) clean

doc:
	@$(CARGO) doc

test: build
	@$(CARGO) test

update:
	@$(CARGO) update

release:
	@$(CARGO) build --release	
	wash claims sign $(RELEASE)/calculator.wasm -c wasmcloud:httpserver --name "calculator" --ver $(VERSION) --rev 0
	wash drain all
	wash reg push localhost:5000/calc:0.1.0 target/wasm32-unknown-unknown/release/calculator_s.wasm --insecure
