#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol, Address, Map,
};

#[contract]
pub struct EscrowCart;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Escrows,
}

#[derive(Clone)]
#[contracttype]
pub struct Escrow {
    pub buyer: Address,
    pub seller: Address,
    pub amount: i128,
    pub released: bool,
}

#[contractimpl]
impl EscrowCart {

    pub fn create_escrow(
        env: Env,
        id: Symbol,
        buyer: Address,
        seller: Address,
        amount: i128,
    ) {
        let mut escrows: Map<Symbol, Escrow> =
            env.storage()
                .instance()
                .get(&DataKey::Escrows)
                .unwrap_or(Map::new(&env));

        escrows.set(
            id.clone(),
            Escrow {
                buyer,
                seller,
                amount,
                released: false,
            },
        );

        env.storage().instance().set(&DataKey::Escrows, &escrows);
    }

    pub fn release(env: Env, id: Symbol, caller: Address) {
        let mut escrows: Map<Symbol, Escrow> =
            env.storage()
                .instance()
                .get(&DataKey::Escrows)
                .unwrap_or(Map::new(&env));

        let mut escrow = escrows.get(id.clone()).expect("not found");

        if caller != escrow.buyer {
            panic!("not authorized");
        }

        escrow.released = true;
        escrows.set(id, escrow);

        env.storage().instance().set(&DataKey::Escrows, &escrows);
    }

    pub fn get(env: Env, id: Symbol) -> Escrow {
        let escrows: Map<Symbol, Escrow> =
            env.storage()
                .instance()
                .get(&DataKey::Escrows)
                .unwrap_or(Map::new(&env));

        escrows.get(id).expect("not found")
    }
}