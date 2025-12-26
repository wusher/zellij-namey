.PHONY: setup build install clean lint test help

WASM_TARGET = wasm32-wasip1
PLUGIN_NAME = zellij_namey.wasm
BUILD_DIR = target/$(WASM_TARGET)/release
INSTALL_DIR = $(HOME)/.config/zellij/plugins

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

setup: ## Install cargo tools (components via rust-toolchain.toml)
	@command -v cargo-llvm-cov >/dev/null || cargo install cargo-llvm-cov

build: ## Build WASM plugin
	cargo build --release --target $(WASM_TARGET)

install: build ## Build and install to ~/.config/zellij/plugins
	mkdir -p $(INSTALL_DIR)
	cp $(BUILD_DIR)/$(PLUGIN_NAME) $(INSTALL_DIR)/

clean: ## Remove build artifacts
	cargo clean

lint: ## Format code and run clippy
	cargo fmt
	cargo clippy -- -D warnings

test: ## Run tests with 95% coverage requirement (excludes plugin glue)
	cargo llvm-cov --target aarch64-apple-darwin --fail-under-lines 95 --ignore-filename-regex 'main\.rs$$'
