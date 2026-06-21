Deploy Savia Carbon Credit Contract

1. Build the contract:
   cd contracts/savia && stellar contract build

2. Fund your Stellar account on testnet:
   https://laboratory.stellar.org/#account-creator?network=testnet

3. Deploy via Stellar CLI:
   stellar contract deploy \
     --wasm target/wasm32v1-none/release/savia.wasm \
     --source <KEY_ALIAS> \
     --network testnet

4. Or use Node.js helper:
   node scripts/deploy.js <YOUR_SECRET_KEY>

---

## Current Deployment

- **Network:** Testnet
- **Contract ID:** `CBBHIK6QE6K6BBDNGOBEXDZJCEZLCLXNNEDS7IEPWFIOQE46D2VN3YL5`
- **Deployer:** `GA7PBYLH364F7BIMAKCTFTDYMF736WJNA5AFLJMI6CLX4J5BLBTCRKHW`
- **WASM Hash:** `16e4e8a4ab0fdc8c966626153d776043f8c14f32443f65d241b6ded85d9ae57d`

## Exported Functions (32)

`add_kyc_verifier`, `add_medical_verifier`, `balance_of`, `check_proof_deadlines`, `create_campaign`, `donate`, `emergency_pause_campaign`, `get_campaign`, `get_campaign_stats`, `get_donation`, `get_donor_dashboard`, `get_donor_nft`, `get_dynamic_nft`, `get_etherfuse_transaction`, `get_kyc_record`, `get_medical_documentation`, `get_peso_exchange_rate`, `get_trust_score`, `initialize`, `initialize_trust_score`, `mint`, `process_refund`, `record_donation`, `register_kyc`, `report_fraud`, `resume_campaign`, `submit_medical_documentation`, `tokens_for_owner`, `update_peso_exchange_rate`, `verify_campaign`, `verify_medical_documentation`

---

## Frontend Deployment (Vercel)

1. Navigate to the frontend directory:
   cd frontend

2. Install dependencies:
   npm install

3. Set up environment variables:
   Copy `.env.example` to `.env.local` and fill in the values.

4. Deploy to Vercel:
   vercel --prod

   Or connect the `savia/frontend` directory to your Vercel project via the Vercel dashboard.
   The `vercel.json` in the frontend directory handles the build configuration automatically.
