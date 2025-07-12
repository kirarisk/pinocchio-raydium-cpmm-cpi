use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

use crate::ID;

/// Holds the current owner of the factory
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Debug, PartialEq)]
pub struct AmmConfig {
    /// Bump to identify PDA
    pub bump: u8,
    /// Status to control if new pool can be create
    pub disable_create_pool: bool,
    /// Config index
    pub index: u16,
    /// The trade fee, denominated in hundredths of a bip (10^-6)
    pub trade_fee_rate: u64,
    /// The protocol fee
    pub protocol_fee_rate: u64,
    /// The fund fee, denominated in hundredths of a bip (10^-6)
    pub fund_fee_rate: u64,
    /// Fee for create a new pool
    pub create_pool_fee: u64,
    /// Address of the protocol fee owner
    pub protocol_owner: Pubkey,
    /// Address of the fund fee owner
    pub fund_owner: Pubkey,
    /// padding
    pub padding: [u64; 16],
}

impl Default for AmmConfig {
    fn default() -> Self {
        Self {
            bump: 0,
            disable_create_pool: false,
            index: 0,
            trade_fee_rate: 0,
            protocol_fee_rate: 0,
            fund_fee_rate: 0,
            create_pool_fee: 0,
            protocol_owner: Pubkey::default(),
            fund_owner: Pubkey::default(),
            padding: [0u64; 16],
        }
    }
}

impl AmmConfig {
    /// The length of the `AmmConfig` account data (including 8-byte discriminator).
    pub const LEN: usize = 8 + 1 + 1 + 2 + 8 + 8 + 8 + 8 + 32 + 32 + (8 * 16);

    /// The 8-byte discriminator for AmmConfig accounts (matches Anchor's discriminator)
    pub const DISCRIMINATOR: &'static [u8] = &[218, 244, 33, 104, 203, 203, 43, 111];

    /// Return a `AmmConfig` from the given account info.
    ///
    /// This method performs owner and length validation on `AccountInfo`, safe borrowing
    /// the account data and deserializing using Borsh.
    #[inline]
    pub fn from_account_info(account_info: &AccountInfo) -> Result<AmmConfig, ProgramError> {
        if !account_info.is_owned_by(&ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        
        let data = account_info.try_borrow_data()?;
        if data.len() < 8 {
            return Err(ProgramError::InvalidAccountData);
        }
        
        // Skip the 8-byte discriminator
        AmmConfig::try_from_slice(&data[8..])
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    /// Return a `AmmConfig` from the given account info.
    ///
    /// This method performs owner and length validation on `AccountInfo`, but does not
    /// perform the borrow check. Uses Borsh deserialization.
    ///
    /// # Safety
    ///
    /// The caller must ensure that it is safe to borrow the account data – e.g., there are
    /// no mutable borrows of the account data.
    #[inline]
    pub unsafe fn from_account_info_unchecked(
        account_info: &AccountInfo,
    ) -> Result<AmmConfig, ProgramError> {
        if account_info.owner() != &ID {
            return Err(ProgramError::InvalidAccountOwner);
        }
        
        let data = account_info.borrow_data_unchecked();
        if data.len() < 8 {
            return Err(ProgramError::InvalidAccountData);
        }
        
        // Skip the 8-byte discriminator
        AmmConfig::try_from_slice(&data[8..])
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    /// Return a `AmmConfig` from the given bytes using Borsh deserialization.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid Borsh-serialized representation of `AmmConfig`.
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Result<AmmConfig, ProgramError> {
        AmmConfig::try_from_slice(bytes)
            .map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        if buf.len() < Self::DISCRIMINATOR.len() {
            return Err(ProgramError::InvalidAccountData);
        }
        
        let given_disc = &buf[..Self::DISCRIMINATOR.len()];
        if Self::DISCRIMINATOR != given_disc {
            return Err(ProgramError::InvalidAccountData);
        }
        
        Self::try_deserialize_unchecked(buf)
    }

    pub fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        let mut data: &[u8] = &buf[Self::DISCRIMINATOR.len()..];
        BorshDeserialize::deserialize(&mut data).map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn discriminator() -> &'static [u8] {
        Self::DISCRIMINATOR
    }

    pub fn owner() -> Pubkey {
        ID
    }

    pub fn size() -> usize {
        Self::LEN
    }
} 