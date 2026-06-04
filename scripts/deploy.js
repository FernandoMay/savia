const sdk = require('@stellar/stellar-sdk');
const { Keypair, TransactionBuilder, Operation, BASE_FEE } = sdk;
const server = new sdk.Horizon.Server('https://horizon-testnet.stellar.org');

const SECRET = process.argv[2] || process.env.STELLAR_SECRET_KEY;

async function deploy() {
  if (!SECRET) {
    const kp = Keypair.random();
    console.log('No secret key provided. Generated new keypair:');
    console.log('Public:', kp.publicKey());
    console.log('Secret:', kp.secret());
    console.log('Fund at: https://laboratory.stellar.org/#account-creator?network=testnet');
    return;
  }
  const kp = Keypair.fromSecret(SECRET);
  console.log('Deploying Savia contracts from:', kp.publicKey());
  // Contract deployment via Soroban CLI:
  // soroban contract deploy --wasm target/wasm32-unknown-unknown/release/savia.wasm --source <SECRET> --network testnet
  console.log('Run: soroban contract deploy --wasm target/wasm32-unknown-unknown/release/savia.wasm --source SECRET --network testnet');
}

deploy();
