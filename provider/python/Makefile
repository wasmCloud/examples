# examples/provider/python/Makefile

PROJECT       = python-provider
CAPABILITY_ID = wasmcloud:example:python
VENDOR        = "wasmcloud"
VERSION       = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION      = 0
oci_url       = localhost:5000/$(PROJECT):$(VERSION)
NAME          = "Python Provider"

# builds are 'release'
# If using debug builds, change bin_path in provider_test_config.toml
TEST_FLAGS := --release

include ../../build/makefiles/provider.mk

ifeq ($(shell nc -czt -w1 127.0.0.1 4222 || echo fail),fail)
test::
	@killall --quiet python-provider
	docker run --rm -d --name provider-test -p 127.0.0.1:4222:4222 nats:2.7.2
	RUST_LOG=info \
	PYTHON_MAIN=$(PWD)/tests \
	PYTHON_DISPATCH=$(PWD)/tests/dispatch.py \
	VENV_PATH=$(PWD)/venv \
		cargo test $(TEST_FLAGS) -- --nocapture
	docker stop provider-test
else
test::
	@killall --quiet python-provider
	RUST_LOG=info \
	PYTHON_MAIN=$(PWD)/tests \
	PYTHON_DISPATCH=$(PWD)/tests/dispatch.py \
	VENV_PATH=$(PWD)/venv \
		cargo test $(TEST_FLAGS) -- --nocapture
endif

