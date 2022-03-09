# Makefile for image uploader

PROJECT  = image_upload
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS   = wasmcloud:httpserver wasmcloud:blobstore wasmcloud:builtin:logging
# registry url for our actor
REG_URL  = localhost:5000/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)

# friendly name for the actor
ACTOR_NAME = "Image uploader"
# optional call alias for actor
#ACTOR_ALIAS=""

include ../../build/makefiles/actor.mk

WASH=wash
HTTPSERVER_REF=wasmcloud.azurecr.io/httpserver:0.15.0
HTTPSERVER_ID=VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M
ACTOR_ID=$(shell make actor_id)

# Change the ID below to the service id from your local build
#BLOBSTORE_REF=localhost:5000/v2/blobstore-s3:0.1.4
BLOBSTORE_ID=VC4FX45M2L5G4DN7FZUBMMVM57WKMVIM4EQSN5UV4PNGZ3ETKLFVGP3F
#$(WASH) ctl start provider $(BLOBSTORE_REF)

run:
	# start http server provider
	$(WASH) ctl start provider $(HTTPSERVER_REF) && sleep 3
	# build this actor and start it
	$(MAKE) all push start && sleep 4
	# link actor with httpserver and blobstore providers
	# (logging is builtin and doesn't need explicit link command)
	$(WASH) ctl link put $(ACTOR_ID) $(HTTPSERVER_ID) wasmcloud:httpserver PORT=8080 && sleep 2
	$(WASH) ctl link put $(ACTOR_ID) $(BLOBSTORE_ID) wasmcloud:blobstore "alias_images=images.from.wasmcloud" && sleep 2


ACTOR_VERSION=$(shell cargo metadata --format-version=1  | jq -r '.packages[17].version')

update:
	$(MAKE)
	$(MAKE) push
	wash ctl update actor \
		NAUOHU25PMHVRPCSSQ3SAGQH26D63R6QEYMI2O2UF7NLQWXFSVJI7KST \
		MCWAGSP5VP3PJCZ47JYJSSTTUYR5ZJ236RHJD7PGMSNKVF7XG4527NCW \
		localhost:5000/v2/image_upload:$(ACTOR_VERSION)

test::
	cargo clippy --all-features --all-targets
