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

We're currently experiencing some build issues related to dependencies and platform tools. Here are the next steps to resolve these issues:

1. **Dependency Management**:
   - We initially tried to use spl-token-2022 but encountered issues with yanked dependencies
   - Currently using the standard SPL token implementation via anchor-spl
   - Will need to revisit Token-2022 integration when dependency issues are resolved

2. **Platform Tools**:
   - Build is failing due to missing platform-tools for ARM64 architecture
   - Need to either use a different build environment or find compatible tools

3. **Testing**:
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
