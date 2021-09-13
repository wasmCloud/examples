# example/provider/factorial/Makefile

CAPABILITY_ID = wasmcloud:example:factorial
NAME = "Factorial"
VENDOR = "wasmcloud"
PROJECT = factorial
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0

include ../../build/makefiles/provider.mk

