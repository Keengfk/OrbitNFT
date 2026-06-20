# 🪐 OrbitNFT

A Soroban smart contract for minting, owning, and transferring Digital Art NFTs natively on the Stellar blockchain.

## 🌟 Overview

OrbitNFT brings true on-chain NFT ownership to Stellar using Soroban smart contracts — replacing simple asset-based NFT tricks with real, enforced ownership logic, transfer rules, and on-chain metadata.

## ✨ Features

- 🎨 Mint NFTs with on-chain metadata (name, description, image URI)
- 👤 Track real ownership per token ID
- 💸 Transfer tokens with authorization checks
- 📊 Query total supply and token metadata
- 🔒 Fully enforced by smart contract code, not convention

## 🛠️ Tech Stack

- **Language:** Rust
- **Platform:** Soroban (Stellar Smart Contracts)
- **Network:** Stellar Testnet

## 🚀 Getting Started

### Prerequisites
- Rust + Cargo
- `wasm32-unknown-unknown` target
- Stellar CLI

### Install dependencies (Linux/Codespaces)

```bash
sudo apt update
sudo apt install -y libdbus-1-dev libudev-dev pkg-config
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli
```

### Build

```bash
stellar contract build
```

### Run tests

```bash
cargo test
```

### Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/nft.wasm \
  --source <your-account> \
  --network testnet
```

## 📋 Contract Functions

| Function | Description |
|----------|-------------|
| `mint` | Mint a new NFT to an address with metadata |
| `owner_of` | Get the current owner of a token |
| `token_metadata` | Get name/description/image of a token |
| `transfer` | Transfer ownership of a token |
| `total_supply` | Get total tokens minted |

## 📁 Project Structure
