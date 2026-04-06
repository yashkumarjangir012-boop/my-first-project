#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct CustomDataContract;

#[contractimpl]
impl CustomDataContract {
    /// Stores a custom string payload mapped to the user's address.
    pub fn store_data(env: Env, user: Address, payload: String) {
        // Guardrail: Verify that the caller is actually the user
        user.require_auth();

        // Save the payload to persistent storage mapped to the user
        env.storage().persistent().set(&user, &payload);
    }

    /// Retrieves the stored payload for a given address.
    pub fn get_data(env: Env, user: Address) -> Option<String> {
        // Fetch data from persistent storage
        env.storage().persistent().get(&user)
    }
}

/* --- Unit Tests --- */
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_store_and_get() {
        let env = Env::default();
        let contract_id = env.register(CustomDataContract, ());
        let client = CustomDataContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let test_payload = String::from_str(&env, "My custom idea data");

        // Soroban requires mock authorization during tests
        env.mock_all_auths();

        client.store_data(&user, &test_payload);
        
        let stored_payload = client.get_data(&user).unwrap();
        assert_eq!(stored_payload, test_payload);
    }
}