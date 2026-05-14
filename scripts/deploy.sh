#!/usr/bin/env bash
# Usage: ./scripts/deploy.sh [testnet|mainnet]
# Requires: stellar CLI, ADMIN_SECRET env var
set -euo pipefail

NETWORK="${1:-testnet}"
WASM_DIR="target/wasm32-unknown-unknown/release"

if [[ -z "${ADMIN_SECRET:-}" ]]; then
  echo "ERROR: ADMIN_SECRET env var not set"
  exit 1
fi

echo "==> Deploying to $NETWORK"

# Helper: deploy a contract and return its ID
deploy_contract() {
  local name="$1"
  local wasm="$WASM_DIR/${name}.wasm"
  echo -n "Deploying $name... "
  stellar contract deploy \
    --wasm "$wasm" \
    --source "$ADMIN_SECRET" \
    --network "$NETWORK"
}

REGISTRY_ID=$(deploy_contract "card_registry")
echo "card_registry: $REGISTRY_ID"

PACK_ID=$(deploy_contract "pack_opening")
echo "pack_opening:  $PACK_ID"

REWARDS_ID=$(deploy_contract "rewards")
echo "rewards:       $REWARDS_ID"

# Derive admin address from secret
ADMIN_ADDR=$(stellar keys address "$ADMIN_SECRET" 2>/dev/null || \
             stellar account address --secret-key "$ADMIN_SECRET")

echo ""
echo "==> Initialising contracts"

# card_registry
stellar contract invoke \
  --id "$REGISTRY_ID" \
  --source "$ADMIN_SECRET" \
  --network "$NETWORK" \
  -- initialize \
  --admin "$ADMIN_ADDR"

# pack_opening (pack price = 1 XLM = 10_000_000 stroops)
stellar contract invoke \
  --id "$PACK_ID" \
  --source "$ADMIN_SECRET" \
  --network "$NETWORK" \
  -- initialize \
  --admin "$ADMIN_ADDR" \
  --registry_contract "$REGISTRY_ID" \
  --pack_price_xlm_stroops 10000000

# rewards (win = 0.5 XLM, win_card=1, draw_card=2)
# XLM native token address on Stellar testnet
XLM_TOKEN="CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"
stellar contract invoke \
  --id "$REWARDS_ID" \
  --source "$ADMIN_SECRET" \
  --network "$NETWORK" \
  -- initialize \
  --admin "$ADMIN_ADDR" \
  --registry_contract "$REGISTRY_ID" \
  --xlm_token "$XLM_TOKEN" \
  --win_xlm_stroops 5000000 \
  --win_card_id 1 \
  --draw_card_id 2

echo ""
echo "==> Deployment complete"
echo "REGISTRY_ID=$REGISTRY_ID"
echo "PACK_ID=$PACK_ID"
echo "REWARDS_ID=$REWARDS_ID"

# Save contract IDs for frontend/backend consumption
cat > .env.contracts <<EOF
REGISTRY_CONTRACT=$REGISTRY_ID
PACK_CONTRACT=$PACK_ID
REWARDS_CONTRACT=$REWARDS_ID
NETWORK=$NETWORK
EOF
echo "Contract IDs saved to .env.contracts"
