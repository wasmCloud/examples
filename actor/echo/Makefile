# examples/actor/echo

KEYDIR ?= .keys
CARGO ?= cargo
REVISION := 0
RUST_DEPS := Cargo.toml Makefile $(wildcard src/*.rs) .cargo/config.toml
TARGET := target/wasm32-unknown-unknown
VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)

# project is a short slug name that will be used in the final file name
PROJECT := echo
# list of all contract claims for actor signing (space-separated)
CLAIMS  := wasmcloud:httpserver
DIST_WASM := build/$(PROJECT)_s.wasm

all: release $(DIST_WASM) $(DIST_EXS)

release:
		cargo build --release

wasm: build/$(PROJECT)_s.wasm

$(DIST_WASM) : $(TARGET)/release/$(PROJECT)_s.wasm
	@mkdir -p $(dir $@)
	cp -p $< $@


.ONESHELL:
build/stub.exs: $(DIST_WASM)
	@mkdir -p $(dir $@)
	@cat <<- EOF > $@
	%{
		name: "$(PROJECT)",
		path: "$(abspath $(DIST_WASM))",
	    key: "$(shell wash claims inspect $(DIST_WASM) -o json | jq -r .module )",
	},
	EOF

$(TARGET)/release/$(PROJECT).wasm: $(RUST_DEPS) Makefile
	$(CARGO) build --release

# sign it (debug or release)
%/$(PROJECT)_s.wasm: %/$(PROJECT).wasm Makefile
	wash claims sign $< \
		$(foreach claim,$(CLAIMS), -c $(claim) ) \
		--name "$(PROJECT)" --ver $(VERSION) --rev $(REVISION)

check clippy doc test update:
	$(CARGO) $@

clean:
	$(CARGO) clean
	rm -rf build

claims: $(DIST_WASM)
	wash claims inspect $(DIST_WASM)

actor_id: $(DIST_WASM)
	wash claims inspect $(DIST_WASM) -o json | jq -r .module

.PHONY: check clean doc test update
