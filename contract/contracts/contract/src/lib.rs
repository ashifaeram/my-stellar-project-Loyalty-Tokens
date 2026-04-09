#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Address};

#[contracttype]
pub enum DataKey {
    Balance(Address),
    Admin,
}

#[contract]
pub struct LoyaltyToken;

#[contractimpl]
impl LoyaltyToken {

    // Initialize contract with admin
    pub fn initialize(env: Env, admin: Address) {
        let storage = env.storage().instance();

        if storage.has(&DataKey::Admin) {
            panic!("Already initialized");
        }

        storage.set(&DataKey::Admin, &admin);
    }

    // Mint tokens (only admin)
    pub fn mint(env: Env, to: Address, amount: i128) {
        let storage = env.storage().instance();

        let admin: Address = storage.get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let key = DataKey::Balance(to.clone());
        let balance: i128 = storage.get(&key).unwrap_or(0);

        storage.set(&key, &(balance + amount));
    }

    // Transfer tokens
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        let storage = env.storage().instance();

        from.require_auth();

        let from_key = DataKey::Balance(from.clone());
        let to_key = DataKey::Balance(to.clone());

        let from_balance: i128 = storage.get(&from_key).unwrap_or(0);
        let to_balance: i128 = storage.get(&to_key).unwrap_or(0);

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        storage.set(&from_key, &(from_balance - amount));
        storage.set(&to_key, &(to_balance + amount));
    }

    // Check balance
    pub fn balance(env: Env, user: Address) -> i128 {
        let storage = env.storage().instance();

        let key = DataKey::Balance(user);
        storage.get(&key).unwrap_or(0)
    }
}