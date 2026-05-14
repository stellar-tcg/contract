# TCG Contracts — Stellar Soroban Smart Contracts

Soroban smart contracts powering the on-chain layer of a blockchain trading card game built on Stellar. Cards are tokenized assets, tradable on Stellar's native DEX, with real player ownership enforced on-chain.

---

## Contracts

| Contract | Description |
|---|---|
| `card_registry` | Core ownership layer — register card types, mint, burn, and transfer cards |
| `pack_opening` | Randomised pack minting with weighted rarity distribution |
| `rewards` | Battle reward distribution — mints cards and pays XLM to winners |

### Rarity Distribution (per card in a pack)

| Rarity | Probability |
|---|---|
| Common | 70% |
| Rare | 20% |
| Epic | 8% |
| Legendary | 2% |

---

## Project Structure

```
contracts/
├── card_registry/      # Card ownership, mint, burn, transfer
├── pack_opening/       # Pack opening with rarity rolls
└── rewards/            # Post-battle reward distribution

packages/
└── shared_types/       # Shared types: CardDef, Rarity, Error, BalanceKey

scripts/
└── deploy.sh           # Deploy + initialise all contracts to testnet/mainnet
```

---

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli) for WASM builds and deployment

---

## Build & Test

```bash
# Run all tests (native host target)
cargo test

# Build optimised WASM for all contracts
make wasm

# Or build a single contract
stellar contract build --package card_registry
```

---

## Deploy

```bash
export ADMIN_SECRET=<your-stellar-secret-key>

# Deploy to testnet (default)
make deploy-all

# Deploy to mainnet
make deploy-all NETWORK=mainnet
```

Contract IDs are saved to `.env.contracts` after deployment:

```
REGISTRY_CONTRACT=C...
PACK_CONTRACT=C...
REWARDS_CONTRACT=C...
NETWORK=testnet
```

---

## Contract API

### `card_registry`

```rust
initialize(admin: Address)
register_card(card_id: u32, asset_code: Symbol, rarity: Rarity, max_supply: u32)
mint(to: Address, card_id: u32, amount: u32)
burn(owner: Address, card_id: u32, amount: u32)
transfer(from: Address, to: Address, card_id: u32, amount: u32)
balance(owner: Address, card_id: u32) -> u32
card_info(card_id: u32) -> CardDef
all_card_ids() -> Vec<u32>
```

### `pack_opening`

```rust
initialize(admin: Address, registry_contract: Address, pack_price_xlm_stroops: i128)
add_card_to_pool(card_id: u32, rarity: Rarity)
open_pack(buyer: Address) -> Vec<u32>   // mints 5 cards, returns card IDs
pool(rarity: Rarity) -> Vec<u32>
pack_price() -> i128
```

### `rewards`

```rust
initialize(admin: Address, registry_contract: Address, xlm_token: Address,
           win_xlm_stroops: i128, win_card_id: u32, draw_card_id: u32)
distribute_reward(player: Address, outcome: MatchOutcome)  // Win=0, Draw=1
set_win_reward(xlm_stroops: i128, card_id: u32)
set_draw_reward(card_id: u32)
win_reward_stroops() -> i128
```

---

## Architecture

This repo covers the **on-chain layer** only. The full system is a hybrid:

| Feature | On-Chain | Off-Chain |
|---|---|---|
| Card ownership | ✅ | |
| Trading (DEX) | ✅ | |
| Pack opening | ✅ | |
| Rewards | ✅ | |
| Battle logic | | ✅ Game server |
| Matchmaking | | ✅ Game server |
| Animations | | ✅ Game client |

The game backend calls `distribute_reward` after verifying a battle result server-side. Only the admin (game backend key) can mint or distribute rewards — the frontend never has mint authority.

---

## Roadmap

- [x] Phase 1 — Card registry, pack opening, rewards
- [ ] Phase 2 — Tournament contract (brackets, entry fees, prize pool)
- [ ] Phase 3 — Crafting contract (card fusion/upgrades)
- [ ] Phase 4 — Staking contract (passive rewards)
- [ ] Phase 5 — DEX order helpers (sell/buy order wrappers)

---

## Security

- **Admin-only minting** — only the initialised admin address can mint cards or distribute rewards
- **Supply caps** — every card type has a `max_supply` enforced on-chain; minting beyond it reverts
- **Auth required** — all state-changing calls require `require_auth()` on the relevant signer
- **No frontend trust** — battle outcomes are validated server-side before any on-chain reward is issued
