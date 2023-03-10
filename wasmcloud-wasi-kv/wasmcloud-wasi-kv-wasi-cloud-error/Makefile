ifndef VERBOSE
MAKEFLAGS += --no-print-directory
endif

.DEFAULT: all
.PHONY: all help \
				build build-release build-watch help

CARGO ?= cargo
CARGO_WATCH ?= cargo-watch
CARGO_CLIPPY ?= cargo-clippy

BUILD_RELEASE_ARG ?=

all: build

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_\-.*]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

###########
# Tooling #
###########

###############
# Development #
###############

lint:
	@$(CARGO) clippy --all-features --all-targets --workspace --fix

build: ## Build the singular wasm module
	@$(CARGO) build --target wasm32-wasi $(BUILD_RELEASE_ARG)

build-watch: ## Build the project continuously
	@$(CARGO) watch -- $(MAKE) build

build-release: ## Build the singular wasm module
	@$(CARGO) build --release --target wasm32-wasi

build-release-watch: ## Build the singular wasm module
	@$(CARGO) watch -- $(MAKE) build-release
