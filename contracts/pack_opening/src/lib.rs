#![no_std]
//! Pack Opening Contract
//!
//! Rarity distribution per card in a pack:
//!   Common    70%  (roll 0–69)
//!   Rare      20%  (roll 70–89)
//!   Epic       8%  (roll 90–97)
//!   Legendary  2%  (roll 98–99)
//!
//! The contract holds a pool of card IDs per rarity tier.
//! On open_pack it picks CARDS_PER_PACK cards, rolls rarity for each,
//! then picks a random card from that tier's pool and calls card_registry::mint.

use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Env, IntoVal, Symbol, Vec,
};
use shared_types::{Error, Rarity, ADMIN_KEY};

const CARDS_PER_PACK: u32 = 5;

// Storage keys
const REGISTRY_KEY: Symbol = symbol_short!("REGISTRY");
const PACK_PRICE: Symbol  = symbol_short!("PRICE");

// Rarity pool keys
fn rarity_key(r: Rarity) -> Symbol {
    match r {
        Rarity::Common    => symbol_short!("COMMON"),
        Rarity::Rare      => symbol_short!("RARE"),
        Rarity::Epic      => symbol_short!("EPIC"),
        Rarity::Legendary => symbol_short!("LEGEND"),
    }
}

fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&ADMIN_KEY).unwrap()
}

/// Deterministic pseudo-random u64 seeded from ledger + sequence counter.
fn pseudo_random(env: &Env, nonce: u32) -> u64 {
    let seed = env.ledger().timestamp() ^ (nonce as u64 * 0x9e3779b97f4a7c15);
    // xorshift64
    let mut x = seed.wrapping_add(0x6c62272e07bb0142);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

fn roll_rarity(rand: u64) -> Rarity {
    match rand % 100 {
        0..=69  => Rarity::Common,
        70..=89 => Rarity::Rare,
        90..=97 => Rarity::Epic,
        _       => Rarity::Legendary,
    }
}

fn pick_card_from_pool(env: &Env, rarity: Rarity, rand: u64) -> Option<u32> {
    let pool: Vec<u32> = env
        .storage()
        .instance()
        .get(&rarity_key(rarity))
        .unwrap_or(Vec::new(env));
    if pool.is_empty() {
        return None;
    }
    let idx = (rand as u32) % pool.len();
    Some(pool.get(idx).unwrap())
}

#[contract]
pub struct PackOpening;

#[contractimpl]
impl PackOpening {
    pub fn initialize(
        env: Env,
        admin: Address,
        registry_contract: Address,
        pack_price_xlm_stroops: i128,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&ADMIN_KEY) {
            return Err(Error::AlreadyInit);
        }
        env.storage().instance().set(&ADMIN_KEY, &admin);
        env.storage().instance().set(&REGISTRY_KEY, &registry_contract);
        env.storage().instance().set(&PACK_PRICE, &pack_price_xlm_stroops);
        Ok(())
    }

    /// Admin adds a card ID to a rarity pool.
    pub fn add_card_to_pool(env: Env, card_id: u32, rarity: Rarity) -> Result<(), Error> {
        get_admin(&env).require_auth();
        let key = rarity_key(rarity);
        let mut pool: Vec<u32> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Vec::new(&env));
        pool.push_back(card_id);
        env.storage().instance().set(&key, &pool);
        Ok(())
    }

    /// Open a pack for `buyer`. Mints CARDS_PER_PACK cards via card_registry.
    /// Returns the list of card IDs minted.
    pub fn open_pack(env: Env, buyer: Address) -> Result<Vec<u32>, Error> {
        buyer.require_auth();

        let registry: Address = env.storage().instance().get(&REGISTRY_KEY).unwrap();
        let mut minted: Vec<u32> = Vec::new(&env);

        for i in 0..CARDS_PER_PACK {
            let rand = pseudo_random(&env, i);
            let rarity = roll_rarity(rand);

            // Fallback chain: if chosen rarity pool is empty, step down to Common
            let card_id = pick_card_from_pool(&env, rarity, rand)
                .or_else(|| pick_card_from_pool(&env, Rarity::Common, rand))
                .ok_or(Error::CardNotFound)?;

            // Cross-contract call to card_registry::mint
            let _: Result<(), Error> = env.invoke_contract(
                &registry,
                &symbol_short!("mint"),
                soroban_sdk::vec![
                    &env,
                    buyer.clone().into_val(&env),
                    card_id.into_val(&env),
                    1u32.into_val(&env),
                ],
            );

            minted.push_back(card_id);
        }

        Ok(minted)
    }

    // ── Read-only ──────────────────────────────────────────────────────────

    pub fn pool(env: Env, rarity: Rarity) -> Vec<u32> {
        env.storage()
            .instance()
            .get(&rarity_key(rarity))
            .unwrap_or(Vec::new(&env))
    }

    pub fn pack_price(env: Env) -> i128 {
        env.storage().instance().get(&PACK_PRICE).unwrap_or(0)
    }
}

mod test;