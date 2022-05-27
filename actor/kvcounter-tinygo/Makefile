NAME    = kvcounter-tinygo
VERSION = 0.1.0
ACTOR   = kvcounter_tinygo

CLAIMS   = --http_server --keyvalue
MODULE   = build/${ACTOR}_s.wasm
UNSIGNED = build/${ACTOR}.wasm
#OCI_URL  = ghcr.io/${GH_USER}/${ACTOR}_s:${VERSION}
OCI_URL  = localhost:5000/${ACTOR}_s:${VERSION}
# if OCI_URL is localhost (and/or with no tls enabled), add --insecure to PUSH_FLAGS
PUSH_FLAGS = --insecure

wasm: ${MODULE}

${MODULE}: ${UNSIGNED} Makefile
	@mkdir -p $(shell dirname $@)
	wash claims sign $< --destination $@ --name ${NAME} ${CLAIMS} -v ${VERSION}

${UNSIGNED}: $(wildcard *.go) go.mod Makefile
	@mkdir -p $(shell dirname $@)
	tinygo build -o $@ -target wasm -scheduler none -no-debug .

clean:
	rm -f ${MODULE} ${UNSIGNED}
	wash drain lib && wash drain oci

push: ${MODULE}
	wash reg push ${OCI_URL} ${PUSH_FLAGS} ${MODULE}

start:
	wash ctl start actor ${OCI_URL}

test:
	echo Insert tests here

# start httpserver provider
provider:
	wash ctl start provider wasmcloud.azurecr.io/httpserver:0.15.0

# link actor with httperver provider
link:
	wash ctl link put $(shell make --silent actor-id) \
		VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M \
		wasmcloud:httpserver address=127.0.0.1:8085

# stop actor
stop:
	wash ctl stop actor $(shell make --silent host-id) $(shell make --silent actor-id)

# show host's inventory
inventory:
	@wash ctl get inventory $(shell make --silent host-id)

# show actor id and claims on wasm file
inspect: ${MODULE}
	@wash claims inspect ${MODULE}

# show actor's public key
actor-id:
	@echo $(shell make --silent _actor_id 2>/dev/null | tail -1)

# show host key
host-id:
	@wash ctl get hosts -o json | jq -r ".hosts[0].id"

_actor_id: ${MODULE}
	wash claims inspect ${MODULE} -o json | jq -r .module | tail -1

_actor_name:
	@echo ${NAME}

.PHONY: actor-id _actor_id bump clean inventory push start test
