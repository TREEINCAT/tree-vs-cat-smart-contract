# TREEINCAT WORLD Smart Contract

Open-source Solana smart contract for [TREEINCAT WORLD](https://world.treeincat.com) - A multiplayer Web3 MMO where players spawn cats, trees, and fish to create and mint rare NFTs.

## 🎮 About

This Anchor-based Solana program powers the in-game economy for TREEINCAT WORLD: Earth Edition. Players purchase items using the $TREEINCAT token, with 95% of each purchase burned (deflationary) and 5% going to the treasury to cover NFT minting fees.

## 📋 Contract Details

- **Program ID**: `FTUi8srvZA5Gng8sfiDFsdN352ydQPnUABWvVqp5qGqF`
- **Network**: Solana Mainnet-Beta
- **Token**: $TREEINCAT (`FJduhgN3MtPNjsu8cPJCqy9jkCMnZwRRCSd4thsHZRpr`)
- **Framework**: Anchor v0.29.0

## 🔥 Tokenomics

- **Burn Rate**: 95% of all purchases are burned
- **Treasury**: 5% goes to treasury for minting operations
- **Deflationary**: Constant supply reduction through gameplay

## 📦 Contract Functions

### `initialize_game`

Initializes the game state with authority, mint, and treasury accounts.

### `buy_fish`

Purchase fish items (Small: 500 TIC, Medium: 750 TIC, Large: 1000 TIC)

### `buy_cat`

Purchase cat spawns (1000 TIC per cat)

### `buy_tree`

Purchase tree placements (1000 TIC per tree)

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
