Deploy Savia Carbon Credit Contract

1. Build the contract:
   cd contracts/hello-world && make build

2. Fund your Stellar account on testnet:
   https://laboratory.stellar.org/#account-creator?network=testnet

3. Deploy via Soroban CLI:
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/savia.wasm \
     --source <YOUR_SECRET_KEY> \
     --network testnet

4. Or use Node.js helper:
   node scripts/deploy.js <YOUR_SECRET_KEY>
