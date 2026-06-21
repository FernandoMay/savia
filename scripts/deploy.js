const sdk = require('@stellar/stellar-sdk');
const { Keypair, TransactionBuilder, Operation, BASE_FEE } = sdk;
const server = new sdk.Horizon.Server('https://horizon-testnet.stellar.org');

const SECRET = process.argv[2] || process.env.STELLAR_SECRET_KEY;

async function deploy() {
  const wasmPath = 'target/wasm32-unknown-unknown/release/savia.wasm';
  if (!SECRET) {
    const kp = Keypair.random();
    console.log('No secret key provided. Generated new keypair:');
    console.log('Public:', kp.publicKey());
    console.log('Secret:', kp.secret());
    console.log('Fund at: https://laboratory.stellar.org/#account-creator?network=testnet');
    console.log('');
    console.log('Then deploy:');
    console.log(`  soroban contract deploy --wasm ${wasmPath} --source <SECRET> --network testnet`);
    return;
  }
  const kp = Keypair.fromSecret(SECRET);
  console.log('Deploying Savia contracts from:', kp.publicKey());
  // Deploy via Stellar CLI:
  // stellar contract deploy --wasm target/wasm32v1-none/release/savia.wasm --source KEY_ALIAS --network testnet
  console.log('Run:');
  console.log(`  stellar contract deploy --wasm ${wasmPath} --source <KEY_ALIAS> --network testnet`);
  console.log('');
  console.log('Current deployed contract ID: CBBHIK6QE6K6BBDNGOBEXDZJCEZLCLXNNEDS7IEPWFIOQE46D2VN3YL5');
}

deploy();
