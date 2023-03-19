# capability-provider/nats/Makefile

PROJECT = wasi_messaging_nats
CAPABILITY_ID = wasmcloud:wasi:messaging
VENDOR   = "wasmcloud"
NAME = "wasi-cloud messaging over NATS"
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
oci_url  = localhost:5000/v2/$(PROJECT):$(VERSION)
RELEASE = --release


include ./provider.mk

ifeq ($(shell nc -czt -w1 127.0.0.1 4222 || echo fail),fail)
test::
	-@killall wasi_messaging_nats 2&>/dev/null
	docker run --rm -d --name nats-provider-test -p 127.0.0.1:4222:4222 nats:2.8 -js
	cargo test $(RELEASE) -- --nocapture
	docker stop nats-provider-test
else
test::
	-@killall wasi_messaging_nats 2&>/dev/null
	cargo test $(RELEASE) -- --nocapture
endif

