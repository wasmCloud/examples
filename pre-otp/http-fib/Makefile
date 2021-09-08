COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
TARGET = target/wasm32-unknown-unknown
DEBUG = $(TARGET)/debug
RELEASE = $(TARGET)/release
KEYDIR ?= .keys
VERSION = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[].version')
# Increment debug revision by 1 each time, defaulting to zero otherwise
REVISION ?= $(shell test -f $(DEBUG)/http_fib.wasm && wash claims inspect $(DEBUG)/http_fib_s.wasm -o json | jq '.revision')
ifeq ($(REVISION),)
REVISION := -1
endif

.PHONY: all build check clean doc test update

all: build

build:
	@$(CARGO) build
	wash claims sign $(DEBUG)/http_fib.wasm --http_server --keyvalue --name "http_fib" --ver $(VERSION) --rev $$(( $(REVISION) + 1 ))
	wash claims inspect $(DEBUG)/http_fib_s.wasm

run: build
	FIB_ACTOR=$(shell wash claims inspect $(DEBUG)/http_fib_s.wasm -o json | jq '.module') \
	wash up --watch $(DEBUG)/http_fib_s.wasm -m manifest.yaml

check:
	@$(CARGO) check

clean:
	@$(CARGO) clean

doc:
	@$(CARGO) doc

inspect:
	wash claims inspect $(RELEASE)/http_fib_s.wasm

release:
	@$(CARGO) build --release	
	wash claims sign $(RELEASE)/http_fib.wasm --http_server --keyvalue --name "http_fib" --ver $(VERSION) --rev 0
	wash claims inspect $(RELEASE)/http_fib_s.wasm

test: build
	@$(CARGO) test

update:
	@$(CARGO) update