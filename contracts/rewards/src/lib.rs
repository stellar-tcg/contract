#![no_std]
//! Rewards Contract
//!
//! The game backend (admin) calls `distribute_reward` after a verified battle.
//! Winners receive:
//!   - A card mint (via card_registry cross-contract call)
//!   - An XLM token reward (native token transfer from contract vault)
//!
//! Reward tiers by match outcome:
//!   Win  → reward_card_id minted + XLM bonus
//!   Draw → reward_card_id minted (no XLM)

use soroban_sdk::{
    contract, contractimpl, symbol_short, token, Address, Env, IntoVal, Symbol,
};
use shared_types::{Error, ADMIN_KEY};

const REGISTRY_KEY: Symbol = symbol_short!("REGISTRY");
const XLM_TOKEN:    Symbol = symbol_short!("XLM_TOK");
const WIN_REWARD:   Symbol = symbol_short!("WIN_XLM");   // stroops
const DRAW_CARD:    Symbol = symbol_short!("DRAW_CARD");  // card_id for draw
const WIN_CARD:     Symbol = symbol_short!("WIN_CARD");   // card_id for win

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MatchOutcome {
    Win  = 0,
    Draw = 1,
}

impl soroban_sdk::TryFromVal<Env, soroban_sdk::Val> for MatchOutcome {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(_env: &Env, v: &soroban_sdk::Val) -> Result<Self, Self::Error> {
        let n: u32 = soroban_sdk::TryFromVal::try_from_val(_env, v)?;
        match n {
            0 => Ok(MatchOutcome::Win),
            1 => Ok(MatchOutcome::Draw),
            _ => Err(soroban_sdk::ConversionError),
        }
    }
}

impl soroban_sdk::IntoVal<Env, soroban_sdk::Val> for MatchOutcome {
    fn into_val(&self, env: &Env) -> soroban_sdk::Val {
        (*self as u32).into_val(env)
    }
}

fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&ADMIN_KEY).unwrap()
}

#[contract]
pub struct Rewards;

#[contractimpl]
impl Rewards {
    pub fn initialize(
        env: Env,
        admin: Address,
        registry_contract: Address,
        xlm_token: Address,
        win_xlm_stroops: i128,
        win_card_id: u32,
        draw_card_id: u32,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&ADMIN_KEY) {
            return Err(Error::AlreadyInit);
        }
        env.storage().instance().set(&ADMIN_KEY, &admin);
        env.storage().instance().set(&REGISTRY_KEY, &registry_contract);
        env.storage().instance().set(&XLM_TOKEN, &xlm_token);
        env.storage().instance().set(&WIN_REWARD, &win_xlm_stroops);
        env.storage().instance().set(&WIN_CARD, &win_card_id);
        env.storage().instance().set(&DRAW_CARD, &draw_card_id);
        Ok(())
    }

    /// Called by the game backend after a verified battle result.
    /// `outcome`: 0 = Win, 1 = Draw
    pub fn distribute_reward(
        env: Env,
        player: Address,
        outcome: MatchOutcome,
    ) -> Result<(), Error> {
        // Only admin (game backend) can distribute rewards — prevents cheating
        get_admin(&env).require_auth();

        let registry: Address = env.storage().instance().get(&REGISTRY_KEY).unwrap();

        let card_id: u32 = match outcome {
            MatchOutcome::Win  => env.storage().instance().get(&WIN_CARD).unwrap(),
            MatchOutcome::Draw => env.storage().instance().get(&DRAW_CARD).unwrap(),
        };

        // Mint reward card
        let _: Result<(), Error> = env.invoke_contract(
            &registry,
            &symbol_short!("mint"),
            soroban_sdk::vec![
                &env,
                player.clone().into_val(&env),
                card_id.into_val(&env),
                1u32.into_val(&env),
            ],
        );

        // XLM bonus for wins
        if outcome == MatchOutcome::Win {
            let xlm_token: Address = env.storage().instance().get(&XLM_TOKEN).unwrap();
            let win_amount: i128 = env.storage().instance().get(&WIN_REWARD).unwrap_or(0);
            if win_amount > 0 {
                let token_client = token::Client::new(&env, &xlm_token);
                token_client.transfer(
                    &env.current_contract_address(),
                    &player,
                    &win_amount,
                );
            }
        }

        Ok(())
    }

    /// Admin can update reward amounts without redeploying.
    pub fn set_win_reward(env: Env, xlm_stroops: i128, card_id: u32) -> Result<(), Error> {
        get_admin(&env).require_auth();
        env.storage().instance().set(&WIN_REWARD, &xlm_stroops);
        env.storage().instance().set(&WIN_CARD, &card_id);
        Ok(())
    }

    pub fn set_draw_reward(env: Env, card_id: u32) -> Result<(), Error> {
        get_admin(&env).require_auth();
        env.storage().instance().set(&DRAW_CARD, &card_id);
        Ok(())
    }

    // ── Read-only ──────────────────────────────────────────────────────────

    pub fn win_reward_stroops(env: Env) -> i128 {
        env.storage().instance().get(&WIN_REWARD).unwrap_or(0)
    }
}

mod test;