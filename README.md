# Savia Project

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.

## First Version of the Contract

[Savia Contract](https://github.com/FernandoMay/soroban-hello-world)

## First Versión of the Platform

[Savia v1 Github](https://github.com/FernandoMay/saviia)
[Savia - Stellar Crowdfunding Platform](https://saviaw3.netlify.app/)

## Second Version of the Platform

[Savia v2 Github](https://github.com/FernandoMay/savia-stellar-nexus)

## Third Version of the Platform - Official Release

[Savia v3 Github](https://github.com/FernandoMay/savia-stellar-bloom-54)
[Savia - Donar con confianza](https://saviadonations.netlify.app/)

## Mobile Application

[Savia App](https://github.com/FernandoMay/saviaapp)

## Official Repository of Savia Contract

[Savia Contract](https://github.com/FernandoMay/savia)