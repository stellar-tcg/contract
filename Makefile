.PHONY: build test clean deploy-all fmt

NETWORK ?= testnet
CONTRACTS = card_registry pack_opening rewards

build:
	cargo build --release --target wasm32-unknown-unknown

test:
	cargo test

fmt:
	cargo fmt --all

clean:
	cargo clean

# Build optimised WASM for each contract
wasm:
	@for c in $(CONTRACTS); do \
		echo "Building $$c..."; \
		stellar contract build --package $$c; \
	done

# Deploy all contracts to NETWORK (requires ADMIN_SECRET env var)
deploy-all: wasm
	@bash scripts/deploy.sh $(NETWORK)

# Run a single contract's tests
test-%:
	cargo test -p $*
