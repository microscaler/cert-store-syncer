SHELL = /usr/bin/env bash -o pipefail
.SHELLFLAGS = -ec

CARGO_DIR := $(HOME)/.cargo
CARGO_BINS := $(CARGO_DIR)/bin



# Set build time variables including version details
LDFLAGS := $(shell source ./hack/scripts/version.sh; version::ldflags)

PATH := $(abspath $(TOOLS_BIN_DIR)):$(PATH)

#$(TOOLS_BIN_DIR):
#	mkdir -p $@

##@ Linting

.PHONY: rustlint
rustlint: $(CARGO_BINS) ## Lint
	$(CARGO_BINS)/cargo clippy

.PHONY: clean
clean: $(CARGO_BINS) ## clean
	$(CARGO_BINS)/cargo clean



