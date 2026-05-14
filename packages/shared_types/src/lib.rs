#![no_std]
use soroban_sdk::{contracttype, contracterror, symbol_short, Symbol};

/// Card rarity tiers with pack-opening probabilities
#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Rarity {
    Common,    // 70%
    Rare,      // 20%
    Epic,      //  8%
    Legendary, //  2%
}

/// On-chain card definition stored in the registry
#[contracttype]
#[derive(Clone)]
pub struct CardDef {
    pub card_id: u32,
    pub asset_code: Symbol,   // e.g. FIRDRGN (max 12 chars)
    pub rarity: Rarity,
    pub max_supply: u32,
    pub minted: u32,
}

/// Per-player card balance key
#[contracttype]
pub struct BalanceKey {
    pub owner: soroban_sdk::Address,
    pub card_id: u32,
}

/// Shared error codes across contracts
#[contracterror]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    CardNotFound      = 1,
    MaxSupplyReached  = 2,
    InsufficientBal   = 3,
    Unauthorized      = 4,
    InvalidAmount     = 5,
    AlreadyInit       = 6,
}

pub const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
