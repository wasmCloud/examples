# build component demo
#
# 'make'  compiles the rust code to wasm32-wasi
# 'make componentize' compiles and turns into component wasm
# 'make compose' joins components into 'actor.component.wasm'
# 'make all' same as 'make compose'
#

TARGET     = wasm32-wasi
TARGET_OUT = target/$(TARGET)/debug
COMP_OUT   = ./components
ACTOR      = $(COMP_OUT)/actor.component.wasm
WASI       = ./wasi_snapshot_preview1.wasm
WIT        = $(wildcard wit/*.wit)
#WASM_TOOLS = /Users/steve/git/bytecodealliance/wasm-tools/target/debug/wasm-tools
WASM_TOOLS = wasm-tools


all: $(ACTOR)

fname = $(not-dir $(basename $1))

##
## Build (compile rust to TARGET_OUT/*.wasm)
##
build: $(TARGET_OUT)/{guest,adapter_host,adapter_guest}.wasm 

$(TARGET_OUT)/guest.wasm: $(WIT) guest/Cargo.toml guest/src/*.rs 
	cargo build --manifest-path guest/Cargo.toml --target $(TARGET)

$(TARGET_OUT)/adapter_guest.wasm: $(WIT) adapter_guest/Cargo.toml adapter_guest/src/*.rs 
	cargo build --manifest-path adapter_guest/Cargo.toml --target $(TARGET)

$(TARGET_OUT)/adapter_host.wasm: $(WIT) adapter_host/Cargo.toml adapter_host/src/*.rs 
	cargo build --manifest-path adapter_host/Cargo.toml --target $(TARGET)


##
## Componentize: turn wasm into component 
##
componentize: $(COMP_OUT)/{guest,adapter_host,adapter_guest}.component.wasm

$(COMP_OUT)/guest.component.wasm: $(TARGET_OUT)/guest.wasm $(COMP_OUT) $(WASI)
	$(WASM_TOOLS) component new $< -o $@ --adapt $(WASI)

$(COMP_OUT)/adapter_guest.component.wasm: $(TARGET_OUT)/adapter_guest.wasm $(COMP_OUT) $(WASI)
	$(WASM_TOOLS) component new $< -o $@ --adapt $(WASI)

$(COMP_OUT)/adapter_host.component.wasm: $(TARGET_OUT)/adapter_host.wasm $(COMP_OUT) $(WASI)
	$(WASM_TOOLS) component new $< -o $@ --adapt $(WASI)

$(COMP_OUT):
	@mkdir -p $@

##
## Compose
##
compose: $(COMP_OUT)/actor.component.wasm

$(COMP_OUT)/actor.component.wasm: $(COMP_OUT)/adapter_host.component.wasm $(COMP_OUT)/adapter_guest.component.wasm $(COMP_OUT)/guest.component.wasm config.yml $(WASI)
	$(WASM_TOOLS) compose -vv -o $@ -c config.yml $<


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
