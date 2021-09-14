# example/provider/fakepay/Makefile

PROJECT = fakepay
CAPABILITY_ID = wasmcloud:example:fakepay
NAME = "FakePay"
VENDOR = "wasmcloud"
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0

include ../../build/makefiles/provider.mk

