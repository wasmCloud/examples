COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
TARGET = target/wasm32-unknown-unknown
DEBUG = $(TARGET)/debug
RELEASE = $(TARGET)/release
KEYDIR ?= .keys
VERSION = "v0.1.0"

.PHONY: all build clean run pinger ponger

all: build

pinger:
	cd pinger && cargo build 

ponger:
	cd ponger && cargo build

build: pinger ponger 	
	wash claims sign pinger/$(DEBUG)/pinger.wasm --name "pinger" --ver $(VERSION) --rev 0
	wash claims sign ponger/$(DEBUG)/ponger.wasm --name "ponger" -a "wasmcloud/examples/ponger" --ver $(VERSION) --rev 0

clean:
	cd pinger && cargo clean
	cd ponger && cargo clean

makeenv: build
	export PINGER_ACTOR=$(shell wash claims inspect pinger/target/wasm32-unknown-unknown/debug/pinger_s.wasm -o json | jq .module | tr -d "\"")

run: makeenv
	wasmcloud -m ./manifest.yaml