# Smart Contract Verification Guide

This guide will help you verify your TREEINCAT WORLD smart contract on-chain, giving it the "Program Source Verified" badge on Solscan and other explorers.

## Why Verify?

- ✅ **Trust Badge**: Shows "Program Source Verified" on Solscan
- ✅ **Transparency**: Users can audit the source code
- ✅ **Remove Phantom Warning**: Verified programs are less likely to be flagged
- ✅ **Community Trust**: Builds confidence in your dApp

## Prerequisites

1. **Solana CLI**: Install if not already
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
   ```

2. **Solana Verify CLI**: Install the verification tool
   ```bash
   cargo install solana-verify
   ```

3. **Docker**: Required for deterministic builds
   ```bash
   # Install Docker Desktop from https://www.docker.com/products/docker-desktop/
   ```

## Steps to Verify

### 1. Generate Cargo.lock (Required)

The verification process needs `Cargo.lock` to ensure reproducible builds:

```bash
cd /path/to/tree-vs-cat-smart-contract
anchor build
```

This will generate `Cargo.lock` in the root directory.

### 2. Commit Cargo.lock to GitHub

```bash
# Remove Cargo.lock from .gitignore if it's listed
git add Cargo.lock
git commit -m "Add Cargo.lock for verification"
git push origin main
```

### 3. Verify the Program On-Chain

Run the verification command:

```bash
solana-verify verify-from-repo \
  --program-id FTUi8srvZA5Gng8sfiDFsdN352ydQPnUABWvVqp5qGqF \
  --commit-hash $(git rev-parse HEAD) \
  --library-name tic_world \
  --mount-path programs/tic_world \
  https://github.com/TREEINCAT/tree-vs-cat-smart-contract
```

**What this does:**
1. Clones your GitHub repo
2. Builds the program in a Docker container (deterministic build)
3. Compares the build hash with the on-chain program
4. Uploads verification data to Solana

### 4. Verify on Solscan

After successful verification, check:
- Go to: https://solscan.io/account/FTUi8srvZA5Gng8sfiDFsdN352ydQPnUABWvVqp5qGqF
- You should see a **"Program Source Verified"** badge
- The badge includes a link to your GitHub repo

## Troubleshooting

### Error: "Build hash mismatch"

This means the on-chain program was built with different dependencies or settings.

**Solution**: The program must be rebuilt and redeployed using the exact same environment. Since your program is already deployed, you have two options:

1. **Verify with original build environment** (if you have it)
2. **Use the deployed binary as source of truth** and accept it can't be rebuilt deterministically

### Error: "Cargo.lock not found"

Run `anchor build` to generate it, then commit to GitHub.

### Error: "Docker not running"

Start Docker Desktop before running verification.

## Alternative: Manual Verification Documentation

If automated verification fails (common for programs deployed before verification tools existed), you can:

1. **Update README** with clear build instructions
2. **Document deployment process** with exact command used
3. **Provide security audit** if available
4. **Link to source code** prominently in your dApp UI

## Post-Verification

Once verified:

1. ✅ Update your website/docs to mention verification
2. ✅ Contact Phantom at `review@phantom.com` with:
   - Solscan link showing verified badge
   - GitHub repo link
   - Brief description of your dApp
3. ✅ Add verification badge to your README:
   ```markdown
   [![Verified](https://img.shields.io/badge/Solana-Verified-success)](https://solscan.io/account/FTUi8srvZA5Gng8sfiDFsdN352ydQPnUABWvVqp5qGqF)
   ```

## Current Status

- ✅ Source code published on GitHub
- ✅ README with contract details
- ✅ MIT License
- ✅ Build instructions
- ⏳ Cargo.lock needs to be generated and committed
- ⏳ On-chain verification pending

## Resources

- [Solana Program Verification Docs](https://github.com/Ellipsis-Labs/solana-verifiable-build)
- [Solana Verify CLI](https://github.com/Ellipsis-Labs/solana-verify)
- [Solscan](https://solscan.io)

