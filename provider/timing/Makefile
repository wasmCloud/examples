# sleepy Makefile

CAPABILITY_ID = "wasmcloud:timing"
NAME = "timing"
VENDOR = "jclmnop"
PROJECT = wasmcloud-provider-timing
VERSION = 0.1.0
REVISION = 0

include ./provider.mk

test::
	cargo clippy --all-targets --all-features
	cargo test -- --nocapture

