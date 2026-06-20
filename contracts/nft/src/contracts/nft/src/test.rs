#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test_mint_and_owner() {
    let env = Env::default();
    let contract_id = env.register_contract(None, NFTContract);
    let client = NFTContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    env.mock_all_auths();

    let token_id = client.mint(
        &owner,
        &String::from_str(&env, "Sunset in Lagos"),
        &String::from_str(&env, "A beautiful sunset"),
        &String::from_str(&env, "ipfs://example"),
    );

    assert_eq!(token_id, 0);
    assert_eq!(client.owner_of(&token_id), owner);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    let contract_id = env.register_contract(None, NFTContract);
    let client = NFTContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let recipient = Address::generate(&env);
    env.mock_all_auths();

    let token_id = client.mint(
        &owner,
        &String::from_str(&env, "Stellar Art"),
        &String::from_str(&env, "Test piece"),
        &String::from_str(&env, "ipfs://example2"),
    );

    client.transfer(&owner, &recipient, &token_id);
    assert_eq!(client.owner_of(&token_id), recipient);
}

#[test]
fn test_total_supply() {
    let env = Env::default();
    let contract_id = env.register_contract(None, NFTContract);
    let client = NFTContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    env.mock_all_auths();

    client.mint(&owner, &String::from_str(&env, "Art 1"), &String::from_str(&env, "Desc"), &String::from_str(&env, "uri1"));
    client.mint(&owner, &String::from_str(&env, "Art 2"), &String::from_str(&env, "Desc"), &String::from_str(&env, "uri2"));

    assert_eq!(client.total_supply(), 2);
  }
