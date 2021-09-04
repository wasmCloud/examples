# examples/actor/xkcd

PROJECT  = xkcd
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 2
# list of all contract claims for actor signing (space-separated)
CLAIMS   = wasmcloud:httpclient wasmcloud:httpserver wasmcloud:builtin:numbergen
# registry url for our actor
REG_URL  = localhost:5000/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)

include ../../build/makefiles/actor.mk

#
# if you're running local builds you get these numbers from
#    `make inspect` for providers, and `make actor_id` for actors
ACTOR_ID = $(shell make actor_id)
HTTPSERVER_PROVIDER_ID = VDCWPLZCQZ366BEFJSGHCNNUUULKSWT25W5TXZ74RXKHGGKLLY3PERPT
HTTPCLIENT_PROVIDER_ID = VD2K2JRF7MFPGGTBXTIGNOJL354GVHDPSD5LUT65ZQMVMZNPMA2ADTOP

link:
	# link to httpserver and httpclient
	# because numbergen is a builtin, it doesn't require a link command
	wash ctl link --timeout 3 $(ACTOR_ID) \
		$(HTTPSERVER_PROVIDER_ID) wasmcloud:httpserver \
		'config_json={"address":"127.0.0.1:8080"}'
	wash ctl link --timeout 3 $(ACTOR_ID) \
		$(HTTPCLIENT_PROVIDER_ID) wasmcloud:httpclient

