#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

// Define keys for state management
const COUNTER: Symbol = symbol_short!("COUNT");

#[contract]
pub struct InsuranceContract;

#[contractimpl]
impl InsuranceContract {
    /// Initializes a policy for a user. (Only the insurance provider should call this)
    pub fn create_policy(env: Env, insurer: Address, user: Address, payout_amount: i128) {
        // Authenticate the insurer
        insurer.require_auth();

        // Guardrail: Policies must have a positive payout
        if payout_amount <= 0 {
            panic!("Payout amount must be greater than zero");
        }

        // Save policy details mapped to the user's address
        // Structure: (payout_amount, is_claim_approved)
        env.storage().persistent().set(&user, &(payout_amount, false));
    }

    /// Triggers/Approves a claim. (Called by an authorized Oracle or the Insurer)
    pub fn approve_claim(env: Env, authority: Address, user: Address) {
        // Authenticate that a recognized authority is triggering this
        authority.require_auth();

        // Fetch existing policy
        if let Some((amount, _)): Option<(i128, bool)> = env.storage().persistent().get(&user) {
            // Update policy to "Approved"
            env.storage().persistent().set(&user, &(amount, true));
        } else {
            panic!("No policy found for this user");
        }
    }

    /// Allows the user to withdraw their payout once approved.
    pub fn claim_payout(env: Env, user: Address) -> i128 {
        // Authenticate that the user themselves is claiming it
        user.require_auth();

        if let Some((amount, is_approved)): Option<(i128, bool)> = env.storage().persistent().get(&user) {
            if !is_approved {
                panic!("Claim has not been approved yet");
            }

            // Remove the policy from storage to prevent double-spending/double-claiming
            env.storage().persistent().remove(&user);

            // In a production app, you'd trigger a cross-contract token transfer here.
            // Returning the amount as a successful receipt.
            amount
        } else {
            panic!("No active or approved policy found");
        }
    }
}

/* --- Unit Tests --- */
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_insurance_flow() {
        let env = Env::default();
        let contract_id = env.register(InsuranceContract, ());
        let client = InsuranceContractClient::new(&env, &contract_id);

        let insurer = Address::generate(&env);
        let user = Address::generate(&env);
        let payout = 5000;

        env.mock_all_auths();

        // 1. Create Policy
        client.create_policy(&insurer, &user, &payout);

        // 2. Approve Claim
        client.approve_claim(&insurer, &user);

        // 3. User Claims Payout
        let received_payout = client.claim_payout(&user);
        assert_eq!(received_payout, payout);
    }
}