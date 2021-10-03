# Makefile for visits

PROJECT  = visits
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS = wasmcloud:sqldb wasmcloud:builtin:logging
# registry url for our actor
REG_URL  = localhost:5000/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)
CALL_ALIAS = petclinic/visits

include ../../../build/makefiles/actor.mk

link-sqldb:
	wash ctl link put $(shell make --silent actor_id) \
	$(SQLDB_ID) config_b64=$(shell base64 -w0 sql-linkdef.json)
