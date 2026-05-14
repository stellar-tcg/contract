#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec};
use shared_types::{BalanceKey, CardDef, Error, Rarity, ADMIN_KEY};

// Storage key for the list of all card IDs
const CARD_IDS_KEY: &str = "CARD_IDS";

fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&ADMIN_KEY).unwrap()
}

fn get_card(env: &Env, card_id: u32) -> Result<CardDef, Error> {
    env.storage()
        .persistent()
        .get(&card_id)
        .ok_or(Error::CardNotFound)
}

fn save_card(env: &Env, card: &CardDef) {
    env.storage().persistent().set(&card.card_id, card);
}

fn balance_of(env: &Env, owner: &Address, card_id: u32) -> u32 {
    let key = BalanceKey { owner: owner.clone(), card_id };
    env.storage().persistent().get(&key).unwrap_or(0u32)
}

fn set_balance(env: &Env, owner: &Address, card_id: u32, amount: u32) {
    let key = BalanceKey { owner: owner.clone(), card_id };
    env.storage().persistent().set(&key, &amount);
}

#[contract]
pub struct CardRegistry;

#[contractimpl]
impl CardRegistry {
    /// One-time initialisation — sets the admin.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&ADMIN_KEY) {
            return Err(Error::AlreadyInit);
        }
        env.storage().instance().set(&ADMIN_KEY, &admin);
        Ok(())
    }

    /// Admin registers a new card type.
    pub fn register_card(
        env: Env,
        card_id: u32,
        asset_code: Symbol,
        rarity: Rarity,
        max_supply: u32,
    ) -> Result<(), Error> {
        get_admin(&env).require_auth();
        if env.storage().persistent().has(&card_id) {
            return Err(Error::AlreadyInit);
        }
        let card = CardDef { card_id, asset_code, rarity, max_supply, minted: 0 };
        save_card(&env, &card);

        // Track card IDs list
        let mut ids: Vec<u32> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, CARD_IDS_KEY))
            .unwrap_or(Vec::new(&env));
        ids.push_back(card_id);
        env.storage().instance().set(&Symbol::new(&env, CARD_IDS_KEY), &ids);
        Ok(())
    }

    /// Mint `amount` copies of `card_id` to `to`. Called by authorised minters (pack_opening, rewards).
    pub fn mint(env: Env, to: Address, card_id: u32, amount: u32) -> Result<(), Error> {
        get_admin(&env).require_auth();
        if amount == 0 {
            return Err(Error::InvalidAmount);
        }
        let mut card = get_card(&env, card_id)?;
        if card.minted + amount > card.max_supply {
            return Err(Error::MaxSupplyReached);
        }
        card.minted += amount;
        save_card(&env, &card);

        let bal = balance_of(&env, &to, card_id);
        set_balance(&env, &to, card_id, bal + amount);
        Ok(())
    }

    /// Burn `amount` copies from `owner`.
    pub fn burn(env: Env, owner: Address, card_id: u32, amount: u32) -> Result<(), Error> {
        owner.require_auth();
        if amount == 0 {
            return Err(Error::InvalidAmount);
        }
        let bal = balance_of(&env, &owner, card_id);
        if bal < amount {
            return Err(Error::InsufficientBal);
        }
        let mut card = get_card(&env, card_id)?;
        card.minted -= amount;
        save_card(&env, &card);
        set_balance(&env, &owner, card_id, bal - amount);
        Ok(())
    }

    /// Transfer `amount` copies from `from` to `to`.
    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        card_id: u32,
        amount: u32,
    ) -> Result<(), Error> {
        from.require_auth();
        if amount == 0 {
            return Err(Error::InvalidAmount);
        }
        let from_bal = balance_of(&env, &from, card_id);
        if from_bal < amount {
            return Err(Error::InsufficientBal);
        }
        // Verify card exists
        get_card(&env, card_id)?;
        set_balance(&env, &from, card_id, from_bal - amount);
        let to_bal = balance_of(&env, &to, card_id);
        set_balance(&env, &to, card_id, to_bal + amount);
        Ok(())
    }

    // ── Read-only ──────────────────────────────────────────────────────────

    pub fn balance(env: Env, owner: Address, card_id: u32) -> u32 {
        balance_of(&env, &owner, card_id)
    }

    pub fn card_info(env: Env, card_id: u32) -> Result<CardDef, Error> {
        get_card(&env, card_id)
    }

    pub fn all_card_ids(env: Env) -> Vec<u32> {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, CARD_IDS_KEY))
            .unwrap_or(Vec::new(&env))
    }
}

mod test;