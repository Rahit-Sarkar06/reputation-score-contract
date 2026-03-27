#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address};

#[contract]
pub struct ReputationContract;

#[contractimpl]
impl ReputationContract {

    // Set reputation score for a user
    pub fn set_score(env: Env, user: Address, score: i128) {
        user.require_auth(); // Only the user can set their score
        env.storage().instance().set(&user, &score);
    }

    // Get reputation score of a user
    pub fn get_score(env: Env, user: Address) -> i128 {
        env.storage().instance().get(&user).unwrap_or(0)
    }
}
