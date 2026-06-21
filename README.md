# Savìa — Blockchain Medical Crowdfunding

Decentralized medical fundraising platform on **Stellar (Soroban)** with Mexican regulatory compliance (KYC/CURP), dynamic NFT-based donor recognition, and on-chain medical document verification.

## Contract (Stellar Soroban)

The Savìa smart contract powers the entire platform with:
- **Campaign Management** — Create, fund, and withdraw from medical fundraising campaigns
- **KYC/AML Compliance** — Mexican CURP validation, phone verification, tiered KYC levels (1-3)
- **Medical Documentation** — Submit, verify, and track medical documents on-chain with deadlines
- **Dynamic NFTs** — Donation-based tree growth system (Seed → Sprout → Sapling → MightyTree)
- **Trust Scoring** — Reputation system with fraud reporting and community moderation
- **Peso Conversion** — XLM ↔ MXN exchange rate via EtherFuse Stellar integration
- **Proof Deadlines** — Automated fund locking if documentation is overdue; admin extension
- **Emergency Controls** — Pause/resume campaigns, refund processing, contract upgrade

### Deployed
- **Testnet**: `CBBHIK6QE6K6BBDNGOBEXDZJCEZLCLXNNEDS7IEPWFIOQE46D2VN3YL5`

## Frontend (Next.js)

React-based web interface at `frontend/` for donors and campaign creators.

## Structure

```
savia/
├── contracts/     # Soroban smart contract (Rust)
│   └── savia/     # Main contract source
├── frontend/      # Next.js web application
├── scripts/       # Deployment and interaction scripts
├── target/        # Compiled WASM artifacts
```

## Quick Start

```bash
# Build contract
cd contracts/savia
cargo build --target wasm32-unknown-unknown --release

# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/savia.wasm \
  --source <identity>

# Start frontend
cd frontend
npm install
npm run dev
```

## Design System

Uses the **Savia Vitality System**: Coral Heart (#F46F5E) primary, Muted Teal secondary, Newsreader (headings) + Hanken Grotesk (body) typography. See `design/DESIGN.md` for full tokens.
