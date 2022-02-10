# examples/actor/logging

PROJECT  = example_logging
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS   = wasmcloud:builtin:logging
# registry url for our actor
REG_URL  = localhost:5000/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)
ACTOR_NAME = "Logging"

include ../../build/makefiles/actor.mk

ACTOR_ID=$(shell make actor_id 2>/dev/null | tail -1)

run:
	@wash call -o json $(ACTOR_ID) Run '[]'
	@echo Check the wasmcloud host logs to see the output

