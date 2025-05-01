# Florin Token Program (ƒ)

This is the implementation of the Florin token (ƒ) for the ƒXYZ Network, a token that represents financial contributions to the network.

## Overview

The Florin token is designed to be:
- A ledger of financial support
- A prerequisite for deeper membership or governance access
- A primitive tied to dynamic NFTs, vouchers, and other rights
- A transparent accountability layer for contributors, investors, or network agents

## Current Implementation

The current implementation includes:

1. **Initialize Mint** - One-time setup for the Florin token mint
2. **Mint Florin** - Permissioned instruction to mint tokens to contributor accounts with optional metadata
3. **Burn Florin** - Used when contributors withdraw from the network or transfer between contexts

## Technical Details

- Built with Anchor 0.31.1 on Solana
- Uses SPL Token standard with plans to upgrade to Token-2022 in the future
- Includes metadata support for tracking contribution sources

## Development Status

The program is currently in development with the following status:

- [x] Basic program structure
- [x] Initialize mint functionality
- [x] Mint tokens functionality
- [x] Burn tokens functionality
- [ ] Token-2022 extensions (planned for future)
- [ ] Integration with membership program
- [ ] Integration with voucher-manager program
- [ ] Integration with fixie-bot program

## Build Issues and Next Steps

The build issues have been resolved by ensuring compatibility between Solana, Anchor, and Rust versions. Here's the setup that works:

1. **Environment Requirements**:
   - Ubuntu 24.04 LTS (or any system with GLIBC 2.38+)
   - Solana CLI v2.1.5
   - Rust 1.79.0
   - Required packages: `pkg-config`, `libssl-dev`

2. **Setup Instructions**:
   ```bash
   # Install Solana CLI v2.1.5
   sh -c "$(curl -sSfL https://release.anza.xyz/v2.1.5/install)"
   
   # Install Rust and required version
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   rustup install 1.79.0
   
   # Set Rust version for this project
   cd /path/to/florin
   rustup override set 1.79.0
   
   # Install required system packages
   apt update && apt install -y pkg-config libssl-dev
   
   # Build the project
   anchor build
   ```

3. **Known Compatibility Issues**:
   - solana-program v2.2.1 requires Rust 1.79.0+
   - Anchor 0.31.1 needs to be compatible with the Solana version
   - Token Extensions require Solana 1.18+ (which we satisfy with Solana 2.1.5)

4. **Testing**:
   - Need to update test files to match the current implementation
   - Fix TypeScript errors in the test files

## Future Enhancements

1. **Token-2022 Extensions**:
   - Implement permanent freeze capability
   - Add non-transferable option
   - Explore confidential transfers

2. **Metadata Improvements**:
   - Store metadata on-chain instead of just logging it
   - Link to voucher system

3. **Access Control**:
   - Upgrade from single authority to multisig or DAO governance

## Deployment Targets

- Local validator (solana-test-validator)
- Devnet for closed testnet launch
- Mainnet via ƒXYZ token launch pipeline
