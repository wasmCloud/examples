COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
TARGET = target/wasm32-unknown-unknown
DEBUG = $(TARGET)/debug
RELEASE = $(TARGET)/release
KEYDIR ?= .keys
VERSION = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[].version')
CAPABILITIES = -c wasmcloud:keyvalue -c wasmcloud:logging -c wasmcloud:httpserver

.PHONY: all build check clean doc test update
all: build

.PHONY: build
build:
	@$(CARGO) build	
	wash claims sign $(DEBUG)/todo_backend.wasm ${CAPABILITIES} --name "todo-backend" --ver $(VERSION) --rev 0

.PHONY: check
check:
	@$(CARGO) check

.PHONY: clean
clean:
	@$(CARGO) clean

.PHONY: doc
doc:
	@$(CARGO) doc

.PHONY: test
test: build
	@$(CARGO) test

.PHONY: update
update:
	@$(CARGO) update

.PHONY: release
release:
	@$(CARGO) build --release	
	wash claims sign $(RELEASE)/todo_backend.wasm ${CAPABILITIES} --name "todo-backend" --ver $(VERSION) --rev 0