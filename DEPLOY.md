Deploy Savia Carbon Credit Contract

1. Build the contract:
   cd contracts/savia && make build

2. Fund your Stellar account on testnet:
   https://laboratory.stellar.org/#account-creator?network=testnet

3. Deploy via Soroban CLI:
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/savia.wasm \
     --source <YOUR_SECRET_KEY> \
     --network testnet

4. Or use Node.js helper:
   node scripts/deploy.js <YOUR_SECRET_KEY>

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
