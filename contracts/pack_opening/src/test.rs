#![cfg(test)]
use soroban_sdk::{testutils::Address as _, Address, Env};
use crate::{PackOpening, PackOpeningClient};
use shared_types::{Error, Rarity};

fn setup_pack(env: &Env) -> (PackOpeningClient<'static>, Address) {
    let contract_id = env.register_contract(None, PackOpening);
    let client = PackOpeningClient::new(env, &contract_id);
    let admin = Address::generate(env);
    // Use a dummy registry address for unit tests (cross-contract not exercised here)
    let registry = Address::generate(env);
    client.initialize(&admin, &registry, &10_000_000i128); // 1 XLM = 10_000_000 stroops
    (client, admin)
}

#[test]
fn test_add_card_to_pool() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup_pack(&env);

    client.add_card_to_pool(&1u32, &Rarity::Common);
    client.add_card_to_pool(&2u32, &Rarity::Common);
    client.add_card_to_pool(&10u32, &Rarity::Legendary);

    let common_pool = client.pool(&Rarity::Common);
    assert_eq!(common_pool.len(), 2);
    let legend_pool = client.pool(&Rarity::Legendary);
    assert_eq!(legend_pool.len(), 1);
}

#[test]
fn test_double_init_error() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup_pack(&env);
    let admin2 = Address::generate(&env);
    let registry = Address::generate(&env);
    let result = client.try_initialize(&admin2, &registry, &0i128);
    assert_eq!(result, Err(Ok(Error::AlreadyInit)));
}

#[test]
fn test_pack_price() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup_pack(&env);
    assert_eq!(client.pack_price(), 10_000_000i128);
}
