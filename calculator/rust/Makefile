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
	wash claims sign $(DEBUG)/calculator.wasm -c wasmcloud:httpserver --name "calculator" --ver $(VERSION) --rev 0 --destination calculator_s.wasm

start:
	RUST_LOG=debug wasmcloud -m manifest.yaml

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
	wash claims sign $(RELEASE)/calculator.wasm -c wasmcloud:httpserver --name "calculator" --ver $(VERSION) --rev 0 --destination calculator_s.wasm
