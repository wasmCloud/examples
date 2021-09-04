# examples/actor/echo-messaging

PROJECT  = echo_messaging
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS   = wasmcloud:messaging wasmcloud:builtin:logging
# registry url for our actor
REG_URL  = localhost:5000/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)

include ../../build/makefiles/actor.mk

# you can get this provider id by running `make inspect` 
# in the nats capability-provider folder
MESSAGING_PROVIDER_ID = VB4JCG7SLE2OYZ7ZWZWJATWRHFM7BOA6WFTRQMRZN7GNIUIDZ6DRJ5JN

link:
	# link to messaging provider
	wash ctl link --timeout 3 $(shell make actor_id | tail -1) \
		$(MESSAGING_PROVIDER_ID) wasmcloud:messaging \
		'URI=nats://localhost:4222' 'SUBSCRIPTION=demo.echo'
