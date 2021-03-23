COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
TARGET = target/wasm32-unknown-unknown
DEBUG = $(TARGET)/debug
RELEASE = $(TARGET)/release
KEYDIR ?= .keys
VERSION = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[].version')

.PHONY: all build check clean doc test update

all: build

build:
	@$(CARGO) build	
	wash claims sign $(DEBUG)/telnet_channel.wasm -c wasmcloud:messaging -c wasmcloud:telnet -c wasmcloud:logging --name "telnet_channel" --ver $(VERSION) --rev 0

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
	wash claims sign $(RELEASE)/telnet_channel.wasm -c wasmcloud:messaging -c wasmcloud:telnet -c wasmcloud:logging --name "telnet_channel" --ver $(VERSION) --rev 0

run: build
	cd ../../../../capability-providers/telnet; rm telnet.par.gz; make mac
	wash drain all
	RUST_LOG=info wasmcloud -m manifest.yaml