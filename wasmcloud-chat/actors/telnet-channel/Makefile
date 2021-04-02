COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
TARGET = target/wasm32-unknown-unknown
DEBUG = $(TARGET)/debug
RELEASE = $(TARGET)/release
KEYDIR ?= .keys
VERSION = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[].version')
CAPABILITIES = --msg --extras --keyvalue --logging -c wasmcloud:telnet

.PHONY: all build check clean doc test update

all: build

build:
	@$(CARGO) build	
	wash claims sign $(DEBUG)/telnet_channel.wasm $(CAPABILITIES) --name "Telnet Channel" --ver $(VERSION) --rev 0

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
	wash claims sign $(RELEASE)/telnet_channel.wasm $(CAPABILITIES) --name "Telnet Channel" --ver $(VERSION) --rev 0

# The following command can be used for debugging the telnet actor alone
# run: build
# 	cd ../../../../capability-providers/telnet; rm telnet.par.gz; make mac
# 	-wash drain lib
# 	# TODO: make sure redis is running
# 	RUST_LOG=debug,oci_distribution=info \
# 	MESSAGES_ACTOR=MATLXLRSEMTIVXOJH4DN36XJPPQGIBTR2NLRFF2WEZCJOPRNW3IHJTWX \
# 	wasmcloud -m manifest.yaml