.DEFAULT: all
.PHONY: all help \
				check-cargo-watch check-wasmtime \
				lint \
				build build-watch help \
				fused-wit run-fused

CARGO ?= cargo
CARGO_WATCH ?= cargo-watch
WASM_TOOLS ?= wasm-tools
WASMTIME ?= wasmtime

all: fused-wit run-fused

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_\-.*]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

###########
# Tooling #
###########

check-cargo-watch:
	@if [ ! $$(command -v $(CARGO_WATCH)) ]; then \
	 echo "ERROR: cargo-watch not installed (see: https://crates.io/crates/cargo-watch)"; \
	 exit 1; \
	fi

check-wasmtime:
	@if [ ! $$(command -v $(WASMTIME)) ]; then \
	 echo "ERROR: wasmtime not installed (see: https://docs.wasmtime.dev/cli-install.html)"; \
	 exit 1; \
	fi

check-wasmtime-component-model-support:
	@if [ ! $$($(WASMTIME) run --help | grep "component-model") ]; then \
	 echo -e "ERROR: Your installed version of wasmtime does *not* support the component model.\nYou must build wasmtime from scratch with cargo, and include the 'component-model' feature (see: https://github.com/bytecodealliance/wasmtime/blob/main/docs/contributing-building.md).\n\nOnce you've done so, make sure the wasmtime binary ($(WASMTIME)) is on your PATH."; \
	 exit 1; \
	fi

check-wasm-tools:
	@if [ ! $$(command -v $(WASM_TOOLS)) ]; then \
	 echo "ERROR: wasm-tools not installed (see: https://github.com/bytecodealliance/wasm-tools)"; \
	 exit 1; \
	fi

###############
# Development #
###############

# Whether to use a release for componentization
BUILD_RELEASE ?=
ifeq (,$(BUILD_RELEASE))
	CARGO_TARGET_SUFFIX = debug
	BUILD_RELEASE_ARG =
else
	CARGO_TARGET_SUFFIX = release
	BUILD_RELEASE_ARG = --release
endif

lint: ## Lint both projects
	@$(MAKE) -C wasmcloud-wasi-kv-wit lint
	@$(MAKE) -C wasmcloud-wasi-kv-cc lint
	@$(MAKE) -C wasmcloud-wasi-kv-types lint
	@$(MAKE) -C wasmcloud-wasi-kv-wasi-cloud-error lint
	@$(MAKE) -C kv-demo lint

# NOTE: we eschew workspace support for cargo build because both components build *different ways*
# one uses `cargo component` while the other uses wit-bindgen which is a standard cargo build

build: build-all ## Runt the build (debug)
build-release:  ## Run the build (release)
	@$(MAKE) build BUILD_RELEASE=yes

build-all: build-provider-wit build-provider-cc build-consumer-demo
build-all-release:
	@$(MAKE) build-all BUILD_RELEASE=yes

build-consumer-demo: ## Build both WASM modules (cargo component, wit-bindgen)
	@$(MAKE) -C kv-demo build BUILD_RELEASE_ARG=$(BUILD_RELEASE_ARG)

build-provider-cc: ## Build the kv provider with wawsm (cargo component toolchain)
	@$(MAKE) -C wasmcloud-wasi-kv-cc build BUILD_RELEASE_ARG=$(BUILD_RELEASE_ARG)

build-provider-wit: ## Build the kv provider (wit-bindgen toolchain)
	@$(MAKE) -C wasmcloud-wasi-kv-wit build BUILD_RELEASE_ARG=$(BUILD_RELEASE_ARG)

build-wasi-types: ## Build the wasi interface types
	@$(MAKE) -C wasmcloud-wasi-kv-types build BUILD_RELEASE_ARG=$(BUILD_RELEASE_ARG)

build-wasi-wasi-cloud-error: ## Build the wasi interface wasi-cloud-error
	@$(MAKE) -C wasmcloud-wasi-kv-wasi-cloud-error build BUILD_RELEASE_ARG=$(BUILD_RELEASE_ARG)

####################
# Componentization #
####################

BUILD_MODULE_OUTPUT_DIR ?= target/wasm32-wasi/$(CARGO_TARGET_SUFFIX)
BUILD_COMPONENT_OUTPUT_DIR ?= components

BUILD_KV_WASI_CLOUD_ERROR ?= $(BUILD_MODULE_OUTPUT_DIR)/wasi_cloud_error.wasm
BUILD_KV_WASI_CLOUD_ERROR_OUTPUT_COMPONENT_PATH ?= $(BUILD_COMPONENT_OUTPUT_DIR)/wasi-cloud-error.wasm

BUILD_KV_TYPES ?= $(BUILD_MODULE_OUTPUT_DIR)/types.wasm
BUILD_KV_TYPES_OUTPUT_COMPONENT_PATH ?= $(BUILD_COMPONENT_OUTPUT_DIR)/types.wasm

BUILD_KV_PROVIDER ?= $(BUILD_MODULE_OUTPUT_DIR)/keyvalue.wasm
BUILD_KV_PROVIDER_OUTPUT_COMPONENT_PATH ?= $(BUILD_COMPONENT_OUTPUT_DIR)/keyvalue.wasm

BUILD_KV_CONSUMER ?= $(BUILD_MODULE_OUTPUT_DIR)/kv_demo.wasm
BUILD_KV_CONSUMER_OUTPUT_COMPONENT_PATH ?= $(BUILD_COMPONENT_OUTPUT_DIR)/kv_demo.wasm
BUILD_KV_CONSUMER_SIGN_NAME ?= kv_demo
BUILD_KV_CONSUMER_SIGNED_OUTPUT_COMPONENT_PATH ?= $(BUILD_COMPONENT_OUTPUT_DIR)/kv_demo_s.wasm

#BUILD_FUSED_REACTOR_ADAPTER ?= $(BUILD_COMPONENT_OUTPUT_DIR)/wasi_snapshot_preview_reactor/wasi_snapshot_preview1.wasm
BUILD_FUSED_REACTOR_ADAPTER ?= $(BUILD_COMPONENT_OUTPUT_DIR)/wasi_snapshot_preview_reactor_latest/wasi_snapshot_preview1.wasm
BUILD_FUSED_COMMAND_ADAPTER ?= $(BUILD_COMPONENT_OUTPUT_DIR)/wasi_snapshot_preview_command/wasi_snapshot_preview1.wasm
BUILD_FUSED_OUTPUT_MODULE_PATH ?= $(BUILD_COMPONENT_OUTPUT_DIR)/kv_demo.fused.wasm

BUILD_FUSED_SIGN_NAME ?= kv_demo.fused
BUILD_FUSED_OUTPUT_COMPONENT_PATH ?= $(BUILD_COMPONENT_OUTPUT_DIR)/kv_demo_fused_s.wasm

# This should be the path to a wash binary (likely custom built) with component signing support
# see: https://github.com/wasmCloud/wascap/commit/48eafc861099b08a531b7eeb033802ab8a215baf
WASH_WITH_COMPONENT_SUPPORT ?=

#fused-wit: check-wasm-tools build-wasi-wasi-cloud-error build-wasi-types build-provider-wit build-consumer-demo ensure-wash-with-comp-signing ## Build a fused ("macro") wasm module, with wit-bindgen

fused-wit: check-wasm-tools build-provider-wit build-consumer-demo  ## Build a fused ("macro") wasm module, with wit-bindgen
	@echo "[info] Building fused..."
	@echo "---------------------------------------------------"
	@echo " wasi-cloud-error     = [$(BUILD_KV_WASI_CLOUD_ERROR)]"
	@echo " types                = [$(BUILD_KV_TYPES)]"
	@echo " provider             = [$(BUILD_KV_PROVIDER)]"
	@echo " consumer             = [$(BUILD_KV_CONSUMER)]"
	@echo " WASI adapter         = [$(BUILD_FUSED_ADAPTER)]"
	@echo ""
	@echo " module output dir    = [$(BUILD_MODULE_OUTPUT_DIR)]"
	@echo " component output dir = [$(BUILD_COMPONENT_OUTPUT_DIR)]"
	@echo "---------------------------------------------------"

# # Create the component that provides wasi-cloud-error
# > @echo "[info] creating the wasi-cloud-error component..."
# >	@$(WASM_TOOLS) component new \
#		--output $(BUILD_KV_WASI_CLOUD_ERROR_OUTPUT_COMPONENT_PATH) \
#		--adapt	$(BUILD_FUSED_REACTOR_ADAPTER) \
#		$(BUILD_KV_WASI_CLOUD_ERROR)
# > @echo ""
# > @echo "[success] successfully wrote wasi-cloud-error provider WASM module [$(BUILD_KV_WASI_CLOUD_ERROR_OUTPUT_COMPONENT_PATH)]"

# # Create the component that provides wasi types
# > @echo "[info] creating the types component..."
# >	@$(WASM_TOOLS) component new \
#		--output $(BUILD_KV_TYPES_OUTPUT_COMPONENT_PATH) \
#		--adapt	$(BUILD_FUSED_REACTOR_ADAPTER) \
#		$(BUILD_KV_TYPES)
# > @echo ""
# > @echo "[success] successfully wrote wasi types provider WASM module [$(BUILD_KV_TYPES_OUTPUT_COMPONENT_PATH)]"

# Create the component that provides KV
	@echo "[info] creating the kv provider component..."
	@$(WASM_TOOLS) component new \
	--output $(BUILD_KV_PROVIDER_OUTPUT_COMPONENT_PATH) \
	--adapt	$(BUILD_FUSED_REACTOR_ADAPTER) \
	$(BUILD_KV_PROVIDER)
	@echo ""
	@echo "[success] successfully wrote kv provider WASM module [$(BUILD_KV_PROVIDER_OUTPUT_COMPONENT_PATH)]"

# Create the component that consumes KV
	@echo "[info] creating the kv consumer component..."
	@$(WASM_TOOLS) component new \
	--output $(BUILD_KV_CONSUMER_OUTPUT_COMPONENT_PATH) \
	--adapt	$(BUILD_FUSED_REACTOR_ADAPTER) \
	$(BUILD_KV_CONSUMER)
	@echo ""
	@echo "[success] successfully wrote kv consumer WASM module [$(BUILD_KV_CONSUMER_OUTPUT_COMPONENT_PATH)]"
	@echo "[success] signing kv consumer with component-ready wash [$(BUILD_KV_CONSUMER_SIGNED_OUTPUT_COMPONENT_PATH)]"
	$(WASH_WITH_COMPONENT_SUPPORT) claims sign --name $(BUILD_KV_CONSUMER_SIGN_NAME) $(BUILD_KV_CONSUMER_OUTPUT_COMPONENT_PATH)

# Compose in dependencies
	@echo "[info] composing consume module with provider..."
	@$(WASM_TOOLS) compose \
	--search-path $(BUILD_COMPONENT_OUTPUT_DIR) \
	--output $(BUILD_FUSED_OUTPUT_MODULE_PATH) \
	$(BUILD_KV_CONSUMER_OUTPUT_COMPONENT_PATH)

	@echo "[success] signing fused component [$(BUILD_FUSED_OUTPUT_MODULE_PATH)]"
	$(WASH_WITH_COMPONENT_SUPPORT) claims sign \
		--cap "wasmcloud:keyvalue" \
		--name $(BUILD_FUSED_SIGN_NAME) \
		$(BUILD_FUSED_OUTPUT_MODULE_PATH)

# # Show component metadata (fused component)
# > @echo "[info] compiled component metadata (fused/fused):"
# > @$(WASM_TOOLS) metadata show $(BUILD_FUSED_OUTPUT_MODULE_PATH)

fused-wit-watch: ## Build fused WASI components (continuously)
	$(CARGO) watch --ignore=$(BUILD_COMPONENT_OUTPUT_DIR) -- $(MAKE) fused-wit

run-fused: check-wasmtime check-wasmtime-component-model-support
	@echo "[info] Running fused component @ [$(BUILD_FUSED_OUTPUT_MODULE_PATH)]..."
	@$(WASMTIME) run \
		--allow-precompiled \
		--wasm-features all \
		--wasi-modules default \
		$(BUILD_FUSED_OUTPUT_MODULE_PATH)
