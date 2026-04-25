#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Address, Symbol};

    #[test]
    fn test_happy_path() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Track2Earn);
        let client = Track2EarnClient::new(&env, &contract_id);

        let sender = Address::generate(&env);
        let rider = Address::generate(&env);

        let id = client.create_delivery(&sender, &rider);
        client.submit_proof(&id, &rider, &Symbol::new(&env, "proof123"));

        let verified = client.verify_delivery(&id);
        assert!(verified);
    }

    #[test]
    #[should_panic]
    fn test_unauthorized_rider() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Track2Earn);
        let client = Track2EarnClient::new(&env, &contract_id);

        let sender = Address::generate(&env);
        let rider = Address::generate(&env);
        let attacker = Address::generate(&env);

        let id = client.create_delivery(&sender, &rider);
        client.submit_proof(&id, &attacker, &Symbol::new(&env, "fake"));
    }

    #[test]
    fn test_state_storage() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Track2Earn);
        let client = Track2EarnClient::new(&env, &contract_id);

        let sender = Address::generate(&env);
        let rider = Address::generate(&env);

        let id = client.create_delivery(&sender, &rider);

        let delivery: Delivery = env
            .storage()
            .instance()
            .get(&DataKey::Delivery(id))
            .unwrap();

        assert_eq!(delivery.sender, sender);
        assert_eq!(delivery.rider, rider);
    }
}
