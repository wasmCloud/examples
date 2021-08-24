# common makefile rules for building actors
#
# Before including this, your project Makefile should define the following:
#
# Required
# -----------
# PROJECT      - Short name for the project, must be valid filename chars, no spaces
# CLAIMS       - Space-separtaed list of capability contracts to use for signing
#                These should match the capability providers the actor needs to use.
#                For example: 
#                    wasmcloud:httpserver wasmcloud:builtin:logging
# VERSION      - The actor version number, usually semver format, X.Y.Z
# REVISION     - A number that should be incremented with every build,
#                whether or not VERSION has changed
# REG_URL      - Registry url, e.g. 'localhost:5000' or 'wasmcloud.azurecr.io'
# PUSH_REG_CMD - Command to push to registry, for example:
#                    wash reg push --insecure $(REG_URL)
#
# 
# Optional
# -----------
# KEYDIR    - path to private key folder
# CARGO     - cargo binary (name or path), defaults to cargo
# WASH      - wash binary (name or path), defaults to wash
# RUST_DEPS - rust source files
# DIST_WASM - the final file after building and signing
# TARGET_DIR - location of cargo build target folder if not in current dir
#              (if it's in a workspace, it may be elsewhere)
# WASM_TARGET - type of wasm file, defaults to wasm32-unknown-unknown
#

KEYDIR    ?= .keys
CARGO     ?= cargo
WASH      ?= wash
RUST_DEPS ?= Cargo.toml Makefile $(wildcard src/*.rs) .cargo/config.toml
# location of cargo output files
TARGET_DIR ?= target
# location of wasm file after build and signing
DIST_WASM ?= build/$(PROJECT)_s.wasm
WASM_TARGET ?= wasm32-unknown-unknown
UNSIGNED_WASM = $(TARGET_DIR)/$(WASM_TARGET)/release/$(PROJECT).wasm

# verify all required variables are set
check-var-defined = $(if $(strip $($1)),,$(error Required variable "$1" is not defined))

$(call check-var-defined,PROJECT)
$(call check-var-defined,CLAIMS)
$(call check-var-defined,VERSION)
$(call check-var-defined,REVISION)
$(call check-var-defined,REG_URL)
$(call check-var-defined,PUSH_REG_CMD)

all:: $(DIST_WASM)

# default target is signed wasm file
# sign it
$(DIST_WASM): $(UNSIGNED_WASM) Makefile
	@mkdir -p $(dir $@)
	$(WASH) claims sign $< \
		$(foreach claim,$(CLAIMS), -c $(claim) ) \
		--name "$(PROJECT)" --ver $(VERSION) --rev $(REVISION) \
		--destination $@


# the wasm should be rebuilt if any source files change
$(UNSIGNED_WASM): $(RUST_DEPS)
	$(CARGO) build --release


# push signed wasm file to registry
push: $(DIST_WASM)
	$(PUSH_REG_CMD) $(DIST_WASM)

# tell host to start an instance of the actor
start:
	$(WASH) ctl start actor $(REG_URL) --timeout 3

# NOT WORKING - live actor updates not working yet
# update it (should update revision before doing this)
#update:
#	$(PUSH_REG_CMD) $(DIST_WASM)
#	$(WASH) ctl update actor  \
#        $(shell $(WASH) ctl get hosts -o json | jq -r ".hosts[0].id") \
#	    $(shell make actor_id | tail -1) \
#	    $(REG_URL) --timeout 3

inventory:
	$(WASH) ctl get inventory $(shell $(WASH) ctl get hosts -o json | jq -r ".hosts[0].id")

# if this is a test actor, run its start method
# run a test if it's got a start method
test::
	$(WASH) call --test --data test-options.json --rpc-timeout 2 \
	    $(shell make actor_id | tail -1) \
	    Start

# generate release build
release::
	cargo build --release

# standard rust commands
check clippy doc:
	$(CARGO) $@

# remove 
clean::
	$(CARGO) clean
	rm -rf build

claims: $(DIST_WASM)
	$(WASH) claims inspect $(DIST_WASM)

actor_id: $(DIST_WASM)
	$(WASH) claims inspect $(DIST_WASM) -o json | jq -r .module

.PHONY: actor_id check clean clippy doc release test update
