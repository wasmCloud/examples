.DEFAULT: all
.PHONY: all help \
				check-cargo-watch check-cargo-clippy \
				build build-watch help

CARGO ?= cargo
CARGO_WATCH ?= cargo-watch
CARGO_CLIPPY ?= cargo-clippy
CARGO_COMPONENT ?= cargo-component

BUILD_RELEASE_ARG ?=

all: build

help:  ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_\-.*]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

###########
# Tooling #
###########

check-cargo:
ifeq (,$(shell command -v $(CARGO)))
	$(error "cargo not installed (see: https://doc.rust-lang.org/cargo/getting-started/installation.html)")
endif

check-cargo-watch:
ifeq (,$(shell command -v $(CARGO_WATCH)))
	$(error "cargo-watch not installed (see: https://crates.io/crates/cargo-watch")
endif

check-cargo-clippy:
ifeq (,$(shell command -v $(CARGO_CLIPPY)))
	$(error "clippy not installed (see: https://github.com/rust-lang/rust-clippy")
endif

check-cargo-component:
ifeq (,$(shell command -v $(CARGO_COMPONENT)))
	$(error "cargo-component not installed (see: https://github.com/bytecodealliance/cargo-component")
endif

###############
# Development #
###############

WASM_MODULE_NAME ?= wasmcloud-wasi-kv-cc

lint:
	@$(CARGO) clippy --all-features --all-targets --workspace --fix

build: ## Build the singular wasm module
	@$(CARGO) component build $(BUILD_RELEASE_ARG)

build-watch: ## Build the project continuously
	@$(CARGO) watch -- $(MAKE) build
