#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Delivery {
    pub sender: Address,
    pub rider: Address,
    pub proof_hash: Symbol,
    pub completed: bool,
}

#[contracttype]
pub enum DataKey {
    Delivery(u32),
    Counter,
}

#[contract]
pub struct Track2Earn;

#[contractimpl]
impl Track2Earn {

    // Create delivery job and store in contract
    pub fn create_delivery(env: Env, sender: Address, rider: Address) -> u32 {
        sender.require_auth();

        let mut counter: u32 = env.storage().instance().get(&DataKey::Counter).unwrap_or(0);
        counter += 1;

        let delivery = Delivery {
            sender: sender.clone(),
            rider,
            proof_hash: Symbol::new(&env, "none"),
            completed: false,
        };

        env.storage().instance().set(&DataKey::Delivery(counter), &delivery);
        env.storage().instance().set(&DataKey::Counter, &counter);

        counter
    }

    // Rider submits proof (QR/photo hash)
    pub fn submit_proof(env: Env, id: u32, rider: Address, proof_hash: Symbol) {
        rider.require_auth();

        let mut delivery: Delivery = env.storage().instance().get(&DataKey::Delivery(id)).unwrap();

        if delivery.completed {
            panic!("Already completed");
        }

        if delivery.rider != rider {
            panic!("Unauthorized rider");
        }

        delivery.proof_hash = proof_hash;
        delivery.completed = true;

        env.storage().instance().set(&DataKey::Delivery(id), &delivery);
    }

    // Verify delivery status
    pub fn verify_delivery(env: Env, id: u32) -> bool {
        let delivery: Delivery = env.storage().instance().get(&DataKey::Delivery(id)).unwrap();

        let status = delivery.completed;

        env.events().publish(
            (Symbol::new(&env, "verify"), id),
            status
        );

        status
    }

    // Simulated payment trigger (escrow release)
    pub fn release_payment(env: Env, id: u32) {
        let delivery: Delivery = env.storage().instance().get(&DataKey::Delivery(id)).unwrap();

        if !delivery.completed {
            panic!("Delivery not completed");
        }

        // In real app: invoke token transfer (XLM/USDC)
        env.events().publish(
            (Symbol::new(&env, "payment"), id),
            Symbol::new(&env, "released")
        );
    }
}
