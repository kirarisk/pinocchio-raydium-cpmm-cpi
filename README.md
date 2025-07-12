<p align="center">
 <img alt="pinocchio-raydium-cpmm-cpi" src="https://github.com/user-attachments/assets/4048fe96-9096-4441-85c3-5deffeb089a6" height="100"/>
</p>
<h3 align="center">
  <code>pinocchio-raydium-cpmm-cpi</code>
</h3>
<p align="center">
  <a href="https://crates.io/crates/pinocchio-raydium-cpmm-cpi"><img src="https://img.shields.io/crates/v/pinocchio-raydium-cpmm-cpi?logo=rust" /></a>
  <a href="https://docs.rs/pinocchio-raydium-cpmm-cpi"><img src="https://img.shields.io/docsrs/pinocchio-raydium-cpmm-cpi?logo=docsdotrs" /></a>
</p>


## Overview

This crate contains [`pinocchio`](https://crates.io/crates/pinocchio) helpers to perform cross-program invocations (CPIs) for Raydium CPMM instructions.

Each instruction defines a `struct` with the accounts and parameters required. Once all values are set, you can call directly `invoke` or `invoke_signed` to perform the CPI.

This is a `no_std` crate.

> **Note:** The API defined in this crate is subject to change.

## Setup
```
cargo add pinocchio-raydium-cpmm-cpi
```

## Examples

Initializing a CPMM pool:
```rust
// This example assumes that the instruction receives all required accounts
// for pool creation including creator, amm_config, authority, pool_state, etc.
InitializeCpmm {
    creator,
    amm_config,
    authority,
    pool_state,
    token_0_mint,
    token_1_mint,
    lp_mint,
    creator_token_0,
    creator_token_1,
    creator_lp_token,
    token_0_vault,
    token_1_vault,
    create_pool_fee,
    observation_state,
    token_program,
    token_0_program,
    token_1_program,
    associated_token_program,
    system_program,
    rent,
    init_amount_0: 1000000,
    init_amount_1: 1000000,
    open_time: 1640995200, // Unix timestamp
}.invoke()?;
```

Performing a swap with base input:
```rust
// This example assumes that the instruction receives all required accounts
// for swapping including payer, authority, pool_state, token accounts, etc.
SwapBaseInput {
    payer,
    authority,
    amm_config,
    pool_state,
    input_token_account,
    output_token_account,
    input_vault,
    output_vault,
    input_token_program,
    output_token_program,
    input_token_mint,
    output_token_mint,
    observation_state,
    amount_in: 1000000, // Amount of input tokens to swap
    minimum_amount_out: 950000, // Minimum output tokens expected
}.invoke()?;
```

## License

The code is licensed under the [Apache License Version 2.0](../LICENSE)
