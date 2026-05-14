#![cfg(test)]
use soroban_sdk::{testutils::Address as _, Address, Env};
use crate::{Rewards, RewardsClient};
use shared_types::Error;

fn setup_rewards(env: &Env) -> RewardsClient<'static> {
    let contract_id = env.register_contract(None, Rewards);
    let client = RewardsClient::new(env, &contract_id);
    let admin    = Address::generate(env);
    let registry = Address::generate(env);
    let xlm_tok  = Address::generate(env);
    client.initialize(&admin, &registry, &xlm_tok, &5_000_000i128, &1u32, &2u32);
    client
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup_rewards(&env);
    assert_eq!(client.win_reward_stroops(), 5_000_000i128);
}

#[test]
fn test_double_init_error() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup_rewards(&env);
    let a = Address::generate(&env);
    let b = Address::generate(&env);
    let c = Address::generate(&env);
    let result = client.try_initialize(&a, &b, &c, &0i128, &0u32, &0u32);
    assert_eq!(result, Err(Ok(Error::AlreadyInit)));
}

#[test]
fn test_set_win_reward() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup_rewards(&env);
    client.set_win_reward(&20_000_000i128, &99u32);
    assert_eq!(client.win_reward_stroops(), 20_000_000i128);
}
