.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project using cargo
	cargo clean

build: ## Build the project using cargo
	cargo build

lint: ## Lint the project using cargo
	cargo clippy

fmt: ## Format the project using cargo
	cargo fmt

all: ## Use everything at once
	cargo clippy
	cargo fmt
	cargo build

run: ## Run the project
	cargo run