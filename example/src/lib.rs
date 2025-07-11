#![no_std]

use pinocchio::{self, account_info::AccountInfo, nostd_panic_handler, program_error::ProgramError, entrypoint, ProgramResult, pubkey::Pubkey};
use pinocchio_pubkey::declare_id;
pub mod instructions;
pub use instructions::*;

entrypoint!(process_instruction);
nostd_panic_handler!();

declare_id!("ChdQLSF4pxDuiuLKMYh4qdL9mrm3kuUb4ZBwS3A7DdB8");

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8]
) -> ProgramResult {
    match data.split_first(){
        Some((InitializeCPMM::DISCRIMINATOR,data)) => InitializeCPMM::try_from((data,accounts))?.process(),
        Some((DepositCPMM::DISCRIMINATOR,data)) => DepositCPMM::try_from((data,accounts))?.process(),
        Some((Swap::DISCRIMINATOR,data)) => Swap::try_from((data,accounts))?.process(),
        Some((SwapBaseOutputCPMM::DISCRIMINATOR,data)) => SwapBaseOutputCPMM::try_from((data,accounts))?.process(),
        Some((WithdrawCPMM::DISCRIMINATOR,data)) => WithdrawCPMM::try_from((data,accounts))?.process(),
        _ => Err(ProgramError::InvalidAccountData)
    }
}