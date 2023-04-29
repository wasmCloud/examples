# examples/actor/random

PROJECT  = example_random
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS = 
# registry url for our actor
REG_URL  = localhost:5000/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)
ACTOR_NAME = "Random"

include ../../build/makefiles/actor.mk

ACTOR_ID=$(shell make actor_id 2>/dev/null | tail -1)

pick-animal:
	@wash call -o json $(ACTOR_ID) Run \
		'["armadillo","bear","gecko","panda","platypus","sloth"]' \
		| jq -r '.[0]'

pick-a-card:
	@CARD=$$(wash call -o json $(ACTOR_ID) Run \
		'["Ace","2","3","4","5","6","7","8","9","10","Jack","Qween","King"]' \
		| jq -r '.[0]') \
	&& SUIT=$$(wash call -o json $(ACTOR_ID) Run \
		'["Hearts","Clubs","Diamonds","Spades"]' \
		| jq -r '.[0]') \
	&& echo $$CARD of $$SUIT

roll-die:
	@wash call -o json $(ACTOR_ID) Run '[]' \
		| jq -r '.[0]'

