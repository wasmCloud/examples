# examples/actor/xkcd

.PHONY: build
PROJECT  = xkcd
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 2
# list of all contract claims for actor signing (space-separated)
CLAIMS   = wasmcloud:httpclient wasmcloud:httpserver wasmcloud:builtin:numbergen
# registry url for our actor
REG_URL  = localhost:5000/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)
ACTOR_NAME = "XKCD Generator"

include ../../build/makefiles/actor.mk

#
# if you're running local builds you get these numbers from
#    `make inspect` for providers, and `make actor_id` for actors
ACTOR_ID = $(shell make actor_id)
HTTPSERVER_PROVIDER_ID = VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M
HTTPCLIENT_PROVIDER_ID = VCCVLH4XWGI3SGARFNYKYT2A32SUYA2KVAIV2U2Q34DQA7WWJPFRKIKM

build:
	cd ui && npm run build
	wash build


link:
	# link to httpserver and httpclient
	# because numbergen is a builtin, it doesn't require a link command
	wash ctl link put --timeout-ms 4000 $(ACTOR_ID) \
		$(HTTPSERVER_PROVIDER_ID) wasmcloud:httpserver \
		'config_json={"address":"127.0.0.1:8080"}'
	wash ctl link put --timeout-ms 4000 $(ACTOR_ID) \
		$(HTTPCLIENT_PROVIDER_ID) wasmcloud:httpclient

