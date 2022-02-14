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
ACTOR_NAME = "Echo Messaging"

include ../../build/makefiles/actor.mk

# messaging provider id from wasmcloud.azurecr.io/nats
# If you built it from source, get the provider id with `make inspect` in the source folder
MESSAGING_PROVIDER_ID = VADNMSIML2XGO2X4TPIONTIC55R2UUQGPPDZPAVSC2QD7E76CR77SPW7

link:
	# link to messaging provider
	wash ctl link --timeout-ms 3000 $(shell make actor_id | tail -1) \
		$(MESSAGING_PROVIDER_ID) wasmcloud:messaging \
		'URI=nats://localhost:4222' 'SUBSCRIPTION=demo.echo'
