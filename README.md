# Savia

Blockchain crowdfunding platform on Stellar (Soroban) with Mexican compliance (KYC, CURP), dynamic NFT growth system, and medical documentation verification.

## Contract Features

- **Campaign Management** — Create and manage medical fundraising campaigns
- **KYC/AML Compliance** — Mexican CURP validation, phone verification, tiered KYC levels
- **Medical Documentation** — Submit, verify, and track medical documents on-chain
- **Dynamic NFTs** — Donation-based tree growth system (Seed → MightyTree)
- **Trust Scoring** — Reputation system with fraud reporting
- **Peso Conversion** — XLM ↔ MXN exchange via EtherFuse integration
- **Proof Deadlines** — Automated fund locking if documentation is overdue
- **Emergency Controls** — Pause/resume campaigns, refund processing

## Tech Stack

- **Smart Contract:** Soroban SDK 22.0.0 (Rust → WASM)
- **Network:** Stellar
- **Testing:** soroban-sdk testutils

## Development

```bash
cargo test                    # Run tests
cargo build --target wasm32-unknown-unknown --release  # Build contract
```

## Tests

6 tests covering: initialization, KYC registration, campaign creation, donations with peso conversion, dynamic NFT growth, and medical documentation flow.
