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

run: build
	wasmcloud -m ./manifest.yaml

