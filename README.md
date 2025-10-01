# Solana Account Serialization

A Solana program  for understanding how   efficiently  account data serialization and deserialization using `bytemuck` for zero-copy operations.

## Overview

This project showcases how to:
- Define serializable account structures with proper memory layout
- Use Program Derived Addresses (PDAs) for account management
- Implement zero-copy serialization/deserialization
- Handle account rent calculations and validation

## Features

- **Zero-copy serialization** using `bytemuck` crate
- **PDA-based account management** for secure account derivation
- **Rent-exempt account handling** with automatic lamport calculations
- **Versioned data structures** for future compatibility
- **Memory-safe operations** with proper padding and alignment

## Project Structure

```
src/
├── lib.rs          # Module exports
├── entrypoint.rs   # Program entry point
├── instruction.rs  # Main instruction processing
├── state.rs        # Account data structures
└── error.rs        # Custom error types
```

## Account Types

### Config Account
- Stores program configuration (admin, fee)
- Fixed size: 48 bytes
- Rent-exempt account

### UserState Account
- Stores user-specific data (balance, owner)
- Fixed size: 48 bytes
- PDA-derived account

## Usage

The program demonstrates incrementing a user's balance by 1, showcasing:
1. PDA validation
2. Account deserialization
3. Data modification
4. Automatic serialization

## Dependencies

- `solana-program`: Core Solana program SDK
- `bytemuck`: Zero-copy serialization
- `thiserror`: Error handling

## Building

```bash
cargo build-bpf
```

## Key Concepts Demonstrated

- Account data serialization patterns
- PDA derivation and validation
- Rent calculation and validation
- Memory layout optimization
