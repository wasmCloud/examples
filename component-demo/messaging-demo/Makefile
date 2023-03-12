# build component demo
#
# 'make'  compiles the rust code to wasm32-wasi
# 'make componentize' compiles and turns into component wasm
# 'make compose' joins components into 'actor.component.wasm'
# 'make all' same as 'make compose'
#

TARGET     = wasm32-wasi
TARGET_OUT = target/$(TARGET)/debug
COMP_OUT   = target/components
ACTOR      = $(COMP_OUT)/actor.component.wasm
WASI       = ./wasi_snapshot_preview1.wasm

all: $(ACTOR)

##
## Build (compile rust to TARGET_OUT/*.wasm)
##
build: $(TARGET_OUT)/guest.wasm $(TARGET_OUT)/adapter.wasm

$(TARGET_OUT)/guest.wasm: guest/Cargo.toml guest/src/*.rs 
	cargo build --manifest-path guest/Cargo.toml --target $(TARGET)

$(TARGET_OUT)/adapter.wasm: adapter/Cargo.toml adapter/src/*.rs 
	cargo build --manifest-path adapter/Cargo.toml --target $(TARGET)

##
## Componentize: turn wasm into component 
##
componentize: $(COMP_OUT)/guest.component.wasm $(COMP_OUT)/adapter.component.wasm

$(COMP_OUT)/guest.component.wasm: $(TARGET_OUT)/guest.wasm $(COMP_OUT) $(WASI)
	wasm-tools component new $< -o $@ --adapt $(WASI)

$(COMP_OUT)/adapter.component.wasm: $(TARGET_OUT)/adapter.wasm $(COMP_OUT) $(WASI)
	wasm-tools component new $< -o $@ --adapt $(WASI)

$(COMP_OUT):
	@mkdir -p $@

##
## Compose
##
compose: $(COMP_OUT)/actor.component.wasm

$(COMP_OUT)/actor.component.wasm: $(COMP_OUT)/adapter.component.wasm $(COMP_OUT)/guest.component.wasm config.yml $(WASI)
	wasm-tools compose -vv -o $@ -c config.yml $<



##
## Run
##
run: $(ACTOR)
	@echo "Running..."
	@cargo run --manifest-path host/Cargo.toml


clean:
	@rm -rf target 

comp-clean:
	@rm -rf $(COMP_OUT)


.PHONY: clean run build componentize compose
