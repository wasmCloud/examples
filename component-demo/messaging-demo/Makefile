# build component demo
#
# 'make'  compiles the rust code as wasm32-wasi
# 'make componentize' compiles and turns into component wasm
# 'make compose' joins components into 'actor.wasm'
# 'make all' same as 'make compose'
#


#build:
#	@echo "Building..."
#	@cargo build --manifest-path ./guest/Cargo.toml   --target wasm32-wasi
#	@cargo build --manifest-path ./adapter/Cargo.toml --target wasm32-wasi


# build rules "make build"

.PHONY: build
build: target/wasm32-wasi/guest.wasm target/wasm32-wasi/adapter.wasm 

target/wasm32-wasi/guest.wasm:  guest/Cargo.toml guest/src/*.rs
	cargo build --manifest-path ./guest/Cargo.toml --target wasm32-wasi

target/wasm32-wasi/adapter.wasm: adapter/Cargo.toml adapter/src/*.rs
	cargo build --manifest-path ./adapter/Cargo.toml --target wasm32-wasi
	

guest.component.wasm: target/wasm32-wasi/debug/guest.wasm wasi_snapshot_preview1.wasm
	@wasm-tools component new ./target/wasm32-wasi/debug/guest.wasm -o $@ --adapt ./wasi_snapshot_preview1.wasm

adapter.component.wasm: target/wasm32-wasi/debug/adapter.wasm wasi_snapshot_preview1.wasm
	@wasm-tools component new ./target/wasm32-wasi/debug/adapter.wasm -o $@ --adapt ./wasi_snapshot_preview1.wasm


.PHONY: componentize
componentize: guest.component.wasm adapter.component.wasm


actor.wasm: config.yml adapter.component.wasm guest.component.wasm wasi_snapshot_preview1.wasm
	wasm-tools compose -o $@ -c config.yml adapter.component.wasm


.PHONY: compose
compose: actor.wasm

all: build componentize compose

#build-all: both.wasm

.PHONY: run
run:
	@echo "Running..."
	@cargo run --manifest-path ./host/Cargo.toml


.PHONY: clean
clean:
	@rm -rf target *.component.wasm