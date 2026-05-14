#![cfg(test)]
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};
use crate::{CardRegistry, CardRegistryClient};
use shared_types::{Error, Rarity};

fn setup() -> (Env, Address, CardRegistryClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, CardRegistry);
    let client = CardRegistryClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin);
    (env, admin, client)
}

#[test]
fn test_register_and_mint() {
    let (_env, _admin, client) = setup();
    client.register_card(&1u32, &symbol_short!("FIRDRGN"), &Rarity::Legendary, &100u32);
    let info = client.card_info(&1u32);
    assert_eq!(info.minted, 0);
    assert_eq!(info.max_supply, 100);

    let player = Address::generate(&_env);
    client.mint(&player, &1u32, &3u32);
    assert_eq!(client.balance(&player, &1u32), 3u32);
    assert_eq!(client.card_info(&1u32).minted, 3u32);
}

#[test]
fn test_transfer() {
    let (env, _admin, client) = setup();
    client.register_card(&2u32, &symbol_short!("ICETITAN"), &Rarity::Rare, &50u32);
    let alice = Address::generate(&env);
    let bob   = Address::generate(&env);
    client.mint(&alice, &2u32, &5u32);
    client.transfer(&alice, &bob, &2u32, &2u32);
    assert_eq!(client.balance(&alice, &2u32), 3u32);
    assert_eq!(client.balance(&bob,   &2u32), 2u32);
}

#[test]
fn test_burn() {
    let (env, _admin, client) = setup();
    client.register_card(&3u32, &symbol_short!("SHDMAGE"), &Rarity::Epic, &200u32);
    let player = Address::generate(&env);
    client.mint(&player, &3u32, &10u32);
    client.burn(&player, &3u32, &4u32);
    assert_eq!(client.balance(&player, &3u32), 6u32);
    assert_eq!(client.card_info(&3u32).minted, 6u32);
}

#[test]
fn test_max_supply_error() {
    let (env, _admin, client) = setup();
    client.register_card(&4u32, &symbol_short!("RARE1"), &Rarity::Common, &2u32);
    let player = Address::generate(&env);
    client.mint(&player, &4u32, &2u32);
    let result = client.try_mint(&player, &4u32, &1u32);
    assert_eq!(result, Err(Ok(Error::MaxSupplyReached)));
}

#[test]
fn test_insufficient_balance_error() {
    let (env, _admin, client) = setup();
    client.register_card(&5u32, &symbol_short!("CARD5"), &Rarity::Common, &10u32);
    let player = Address::generate(&env);
    client.mint(&player, &5u32, &1u32);
    let result = client.try_burn(&player, &5u32, &5u32);
    assert_eq!(result, Err(Ok(Error::InsufficientBal)));
}

#[test]
fn test_double_init_error() {
    let (env, _admin, client) = setup();
    let admin2 = Address::generate(&env);
    let result = client.try_initialize(&admin2);
    assert_eq!(result, Err(Ok(Error::AlreadyInit)));
}
