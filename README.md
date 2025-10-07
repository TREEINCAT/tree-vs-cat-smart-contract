# TREEINCAT WORLD Smart Contract

Open-source Solana smart contract for [TREEINCAT WORLD](https://world.treeincat.com) - A multiplayer Web3 MMO where players spawn cats, trees, and fish to create and mint rare NFTs.

## 🎮 About

This Anchor-based Solana program powers the in-game economy for TREEINCAT WORLD: Earth Edition. Players purchase items using the $TREEINCAT token, with 95% of each purchase burned (deflationary) and 5% going to the treasury to cover NFT minting fees.

## 📋 Contract Details

- **Program ID**: `FTUi8srvZA5Gng8sfiDFsdN352ydQPnUABWvVqp5qGqF`
  - [View on Solana Explorer](https://explorer.solana.com/address/FTUi8srvZA5Gng8sfiDFsdN352ydQPnUABWvVqp5qGqF)
  - [View on Solscan](https://solscan.io/account/FTUi8srvZA5Gng8sfiDFsdN352ydQPnUABWvVqp5qGqF)
- **Network**: Solana Mainnet-Beta
- **Token**: $TREEINCAT (`5AzTQ5zUuWUNGetxaRpo9DK9w8FQXoPjn5a4ZXMMmoon`)
  - [View on Solana Explorer](https://explorer.solana.com/address/5AzTQ5zUuWUNGetxaRpo9DK9w8FQXoPjn5a4ZXMMmoon)
  - [View on Solscan](https://solscan.io/token/5AzTQ5zUuWUNGetxaRpo9DK9w8FQXoPjn5a4ZXMMmoon)
- **Framework**: Anchor v0.29.0

## 🔒 Security & Transparency

### Deployment History
- **Deployed**: 06/10/2025 on Solana Mainnet
- **Status**: Active and running without security incidents
- **Built From**: Private monorepo containing full game logic
- **This Repo**: Contains the complete smart contract source code for public audit

### Why Not Auto-Verified?
Automated on-chain verification tools currently have limitations with newer Cargo versions. While we cannot use automated verification at this time, we provide complete transparency:

- ✅ **Full Source Code**: Complete contract logic published here
- ✅ **10 Months Live**: Proven track record on mainnet
- ✅ **No Security Issues**: Zero exploits or vulnerabilities reported
- ✅ **On-Chain History**: All transactions visible on [Solscan](https://solscan.io/account/FTUi8srvZA5Gng8sfiDFsdN352ydQPnUABWvVqp5qGqF)
- ✅ **Open Source**: MIT Licensed for community audit
- ✅ **Active Development**: Regular updates and maintenance

### Trust Signals
- Program has processed thousands of transactions safely
- All contract functions are deterministic and auditable
- No upgrade authority (immutable contract)
- Treasury and burn mechanisms are transparent and verifiable on-chain

## 🔥 Tokenomics

- **Burn Rate**: 95% of all purchases are burned
- **Treasury**: 5% goes to treasury for minting operations
- **Deflationary**: Constant supply reduction through gameplay

## 📦 Contract Functions

### `initialize_game`

Initializes the game state with authority, mint, and treasury accounts.

### `buy_fish`

Purchase fish items with variable pricing:

- Small: 500 TIC
- Medium: 750 TIC
- Large: 1000 TIC

### `buy_cat`

Purchase cat spawns with tiered pricing based on rarity:

- Stray: 250 TIC
- Kitten: 400 TIC
- Tabby: 550 TIC
- Persian: 700 TIC
- MaineCoon: 850 TIC
- Siamese: 1000 TIC
- Bengal: 2000 TIC
- ShadowCat: 3000 TIC

### `buy_tree`

Purchase tree placements with tiered pricing based on rarity:

- Seedling: 250 TIC
- Sapling: 400 TIC
- Oak: 550 TIC
- Pine: 700 TIC
- Redwood: 850 TIC
- Willow: 1000 TIC
- Baobab: 1150 TIC
- Palm: 1300 TIC
- Bonsai: 1450 TIC
- Maple: 1600 TIC
- Cactus: 2150 TIC
- WorldTree: 4300 TIC

## 🏗️ Project Structure

```
programs/
  └── tic_world/
      ├── src/
      │   └── lib.rs       # Main contract logic
      └── Cargo.toml       # Program dependencies
Anchor.toml              # Anchor configuration
Cargo.toml               # Workspace configuration
```

## 🔧 Building

Prerequisites:

- Rust 1.75+
- Solana CLI 1.18+
- Anchor CLI 0.29.0

```bash
# Install dependencies
anchor build

# Run tests
anchor test

# Deploy (requires authority keypair)
anchor deploy
```

## 🔐 Security Features

- Authority-only initialization
- Strict ownership validation
- Safe token transfers and burns
- PDA-based state management
- Comprehensive error codes

## 📊 Events

The contract emits events for all purchases:

- `FishPurchased`
- `CatPurchased`
- `TreePurchased`

## 🌐 Links

- **Game**: [world.treeincat.com](https://world.treeincat.com)
- **Website**: [treeincat.com](https://treeincat.com)
- **Twitter**: [@TreeStuckInCat](https://x.com/TreeStuckInCat)
- **Telegram**: [t.me/TreeInCat](https://t.me/TreeInCat)

## 📄 License

MIT License - See LICENSE file for details

## 🤝 Contributing

This is the production smart contract. For security reasons, we do not accept pull requests. If you find a security vulnerability, please contact us privately.

## ⚠️ Disclaimer

This smart contract is provided as-is. Use at your own risk. Always verify contract addresses before interacting with them.

---

Built with ❤️ by the TREEINCAT team
