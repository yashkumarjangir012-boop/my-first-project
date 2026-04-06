📝 Project Description
This project aims to eliminate the traditional "wait-and-verify" lag in insurance settlements. By utilizing the Stellar blockchain, this contract creates a trustless environment where insurance policies are cryptographically bound to user addresses, and payouts are triggered instantly upon verification by authorized data sources (oracles).

⚙️ What it does
The contract manages the lifecycle of an insurance policy in three distinct phases:

Issuance: The Insurer registers a policy for a specific Address with a defined payout amount.

Verification: An authorized authority (like a weather oracle or flight data feed) signs a transaction to approve a claim based on real-world events.

Settlement: The user triggers the payout, and the contract validates the approval and releases the funds (or reports the amount) before clearing the state to prevent double-claiming.

✨ Features
Trustless Settlement: No manual intervention is required once a claim is approved by the authorized oracle.

Identity-First Security: Leveraging Soroban’s require_auth() to ensure only the policyholder can claim their funds.

State Efficiency: Optimized for the Stellar ledger by using persistent storage and clearing data post-claim to minimize storage costs.

Extensible Logic: Designed to easily integrate with standard Stellar assets (SEP-41) for automated token transfers.

🔗 Deployed Smart Contract Link
Network: Stellar Testnet

Contract ID: Insurance Claim Automation (Replace with your actual Hex ID, e.g., CC...) Explorer: View on StellarExpert

🛠 Developer Setup
Prerequisites
Rust

Soroban CLI

Target wasm32-unknown-unknown

Installation & Build
Clone the repo:

Bash
git clone https://github.com/yourusername/insurance-automation.git
cd insurance-automation
Build the WASM binary:

Bash
soroban contract build
Run Tests:

Bash
cargo test
🚀 Usage Guide
To interact with the contract via the CLI:

Bash
# Create a policy
soroban contract invoke --id <CA7G4ACD5T52CQBLXSKBQPRIOY3H7FNBI26LSP74IKXKHETCMAP7AXBD> --source-account <INSURER> --network testnet -- \
  create_policy --insurer <INSURER_ADDR> --user <USER_ADDR> --payout_amount 1000

# Approve a claim (as an Oracle)
soroban contract invoke --id <CONTRACT_ID> --source-account <ORACLE> --network testnet -- \
  approve_claim --authority <ORACLE_ADDR> --user <USER_ADDR>
  1. Contract address:CA7G4ACD5T52CQBLXSKBQPRIOY3H7FNBI26LSP74IKXKHETCMAP7AXBD
	2. Wallet address:GBEWHGNM4F5HJR4E2BENSXES2CIHXURCN2JFSE7XBNNPYXX6GSZJ6F23
	3. Image:<img width="959" height="425" alt="image" src="https://github.com/user-attachments/assets/323f1cad-2c2e-4f91-bfd3-8a8d5efcd2d4" />

	4. Link:https://stellar.expert/explorer/testnet/contract/CA7G4ACD5T52CQBLXSKBQPRIOY3H7FNBI26LSP74IKXKHETCMAP7AXBD
