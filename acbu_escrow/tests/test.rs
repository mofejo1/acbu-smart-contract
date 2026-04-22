#![cfg(test)]

use acbu_escrow::{Escrow, EscrowClient};
use soroban_sdk::{testutils::Address as _, Address, Env, Error};

#[test]
fn test_release_missing_escrow_returns_not_found() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let payer = Address::generate(&env);
    let acbu_token = env.register_stellar_asset_contract_v2(admin.clone()).address();

    let contract_id = env.register_contract(None, Escrow);
    let client = EscrowClient::new(&env, &contract_id);

    client.initialize(&admin, &acbu_token);

    let result = client.try_release(&1u64, &payer);
    assert_eq!(
        result,
        Err(Ok(Error::from_contract_error(3003)))
    );
}

#[test]
fn test_refund_missing_escrow_returns_not_found() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let payer = Address::generate(&env);
    let acbu_token = env.register_stellar_asset_contract_v2(admin.clone()).address();

    let contract_id = env.register_contract(None, Escrow);
    let client = EscrowClient::new(&env, &contract_id);

    client.initialize(&admin, &acbu_token);

    let result = client.try_refund(&1u64, &payer);
    assert_eq!(
        result,
        Err(Ok(Error::from_contract_error(3003)))
    );
}

#[test]
fn test_pause_without_initialize_returns_uninitialized_admin_error() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, Escrow);
    let client = EscrowClient::new(&env, &contract_id);

    let result = client.try_pause();
    assert_eq!(
        result,
        Err(Ok(Error::from_contract_error(3006)))
    );
}
