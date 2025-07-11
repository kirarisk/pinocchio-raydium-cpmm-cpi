pub mod amm_config;
pub mod pool_state;
pub mod observation;
pub mod observation_state;

pub use amm_config::*;
pub use pool_state::*;
pub use observation::*;
pub use observation_state::*;

/// Seed to derive account address and signature
pub const POOL_SEED: &str = "pool";
pub const POOL_LP_MINT_SEED: &str = "pool_lp_mint";
pub const POOL_VAULT_SEED: &str = "pool_vault";
pub const OBSERVATION_SEED: &str = "observation";
pub const AMM_CONFIG_SEED: &str = "amm_config";

// Number of ObservationState element
pub const OBSERVATION_NUM: usize = 100;
pub const OBSERVATION_UPDATE_DURATION_DEFAULT: u64 = 15;

pub const Q32: u128 = (u32::MAX as u128) + 1; // 2^32

pub enum PoolStatusBitIndex {
    Deposit,
    Withdraw,
    Swap,
}

#[derive(PartialEq, Eq)]
pub enum PoolStatusBitFlag {
    Enable,
    Disable,
}