# Makefile for customers

PROJECT  = customers
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS   = wasmcloud:sqldb wasmcloud:builtin:logging
# registry url for our actor
REG_SERVER ?= localhost:5000
REG_URL  = $(REG_SERVER)/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)
ACTOR_ALIAS = petclinic/customers
ACTOR_NAME = "Pet Clinic Customers"

include ../../../build/makefiles/actor.mk

