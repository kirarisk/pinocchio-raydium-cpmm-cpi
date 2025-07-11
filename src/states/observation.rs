/// The element of observations in ObservationState
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Observation {
    /// The block timestamp of the observation
    pub block_timestamp: u64,
    /// the cumulative of token0 price during the duration time, Q32.32, the remaining 64 bit for overflow
    pub cumulative_token_0_price_x32: u128,
    /// the cumulative of token1 price during the duration time, Q32.32, the remaining 64 bit for overflow
    pub cumulative_token_1_price_x32: u128,
}

impl Observation {
    /// The length of the `Observation` data.
    pub const LEN: usize = core::mem::size_of::<Observation>();
} 