use pinocchio::{
    account_info::{AccountInfo, Ref},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::ID;
use super::{Observation, OBSERVATION_NUM};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObservationState {
    /// Whether the ObservationState is initialized
    pub initialized: bool,
    /// the most-recently updated index of the observations array
    pub observation_index: u16,
    pub pool_id: Pubkey,
    /// observation array
    pub observations: [Observation; OBSERVATION_NUM],
    /// padding for feature update
    pub padding: [u64; 4],
}

impl ObservationState {
    /// The length of the `ObservationState` account data.
    pub const LEN: usize = core::mem::size_of::<ObservationState>();

    /// Anchor-compatible discriminator
    pub const DISCRIMINATOR: &'static [u8] = &[122, 174, 197, 53, 129, 9, 165, 132];

    /// Return a `ObservationState` from the given account info.
    ///
    /// This method performs owner and length validation on `AccountInfo`, safe borrowing
    /// the account data.
    #[inline]
    pub fn from_account_info(account_info: &AccountInfo) -> Result<Ref<ObservationState>, ProgramError> {
        if !account_info.is_owned_by(&ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(Ref::map(account_info.try_borrow_data()?, |data| unsafe {
            Self::from_bytes(&data[8..])
        }))
    }

    /// Return a `ObservationState` from the given account info.
    ///
    /// This method performs owner and length validation on `AccountInfo`, but does not
    /// perform the borrow check.
    ///
    /// # Safety
    ///
    /// The caller must ensure that it is safe to borrow the account data – e.g., there are
    /// no mutable borrows of the account data.
    #[inline]
    pub unsafe fn from_account_info_unchecked(
        account_info: &AccountInfo,
    ) -> Result<&Self, ProgramError> {
        if account_info.data_len() != Self::LEN + 8 {
            return Err(ProgramError::InvalidAccountData);
        }
        if account_info.owner() != &ID {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(Self::from_bytes(&account_info.borrow_data_unchecked()[8..]))
    }

    /// Return a `ObservationState` from the given bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of `ObservationState`.
    #[inline(always)]
    pub unsafe fn from_bytes(bytes: &[u8]) -> &Self {
        &*(bytes.as_ptr() as *const ObservationState)
    }

    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        self.initialized
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
        let data: &[u8] = &buf[Self::DISCRIMINATOR.len()..];
        if data.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        
        unsafe {
            Ok(*Self::from_bytes(data))
        }
    }

    pub fn discriminator() -> &'static [u8] {
        Self::DISCRIMINATOR
    }

    pub fn owner() -> Pubkey {
        ID
    }

    pub fn size() -> usize {
        Self::LEN + 8
    }
} 