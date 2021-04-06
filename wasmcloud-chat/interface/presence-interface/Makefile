.PHONY: all deps codegen build clean

all: deps codegen build

deps:

codegen:
	wapc generate codegen.yaml

build:
	cargo build

# Rust builds accrue disk space over time (specifically the target directory),
# so running `make clean` should be done periodically.
clean:
	cargo clean
	rm -Rf build