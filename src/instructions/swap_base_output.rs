use core::slice::from_raw_parts;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};

use crate::{write_bytes, UNINIT_BYTE};

/// Swap the tokens in the pool base output amount
///
/// ### Accounts:
///   0. `[SIGNER]` payer - The user performing the swap
///   1. `[]` authority - Pool vault and lp mint authority (PDA)
///   2. `[]` amm_config - The factory state to read protocol fees
///   3. `[WRITE]` pool_state - The program account of the pool in which the swap will be performed
///   4. `[WRITE]` input_token_account - The user token account for input token
///   5. `[WRITE]` output_token_account - The user token account for output token
///   6. `[WRITE]` input_vault - The vault token account for input token
///   7. `[WRITE]` output_vault - The vault token account for output token
///   8. `[]` input_token_program - SPL program for input token transfers
///   9. `[]` output_token_program - SPL program for output token transfers
///   10. `[]` input_token_mint - The mint of input token
///   11. `[]` output_token_mint - The mint of output token
///   12. `[WRITE]` observation_state - The program account for the most recent oracle observation
pub struct SwapBaseOutput<'a> {
    /// The user performing the swap
    pub payer: &'a AccountInfo,
    /// CHECK: pool vault and lp mint authority
    pub authority: &'a AccountInfo,
    /// The factory state to read protocol fees
    pub amm_config: &'a AccountInfo,
    /// The program account of the pool in which the swap will be performed
    pub pool_state: &'a AccountInfo,
    /// The user token account for input token
    pub input_token_account: &'a AccountInfo,
    /// The user token account for output token
    pub output_token_account: &'a AccountInfo,
    /// The vault token account for input token
    pub input_vault: &'a AccountInfo,
    /// The vault token account for output token
    pub output_vault: &'a AccountInfo,
    /// SPL program for input token transfers
    pub input_token_program: &'a AccountInfo,
    /// SPL program for output token transfers
    pub output_token_program: &'a AccountInfo,
    /// The mint of input token
    pub input_token_mint: &'a AccountInfo,
    /// The mint of output token
    pub output_token_mint: &'a AccountInfo,
    /// The program account for the most recent oracle observation
    pub observation_state: &'a AccountInfo,
    
    /// Instruction parameters
    pub max_amount_in: u64,
    pub amount_out: u64,
}

impl SwapBaseOutput<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        // Account metadata - 13 accounts total
        let account_metas: [AccountMeta; 13] = [
            AccountMeta::writable_signer(self.payer.key()),
            AccountMeta::readonly(self.authority.key()),
            AccountMeta::readonly(self.amm_config.key()),
            AccountMeta::writable(self.pool_state.key()),
            AccountMeta::writable(self.input_token_account.key()),
            AccountMeta::writable(self.output_token_account.key()),
            AccountMeta::writable(self.input_vault.key()),
            AccountMeta::writable(self.output_vault.key()),
            AccountMeta::readonly(self.input_token_program.key()),
            AccountMeta::readonly(self.output_token_program.key()),
            AccountMeta::readonly(self.input_token_mint.key()),
            AccountMeta::readonly(self.output_token_mint.key()),
            AccountMeta::writable(self.observation_state.key()),
        ];

        // Instruction data layout:
        // -  [0..8]: instruction discriminator (8 bytes)
        // -  [8..16]: max_amount_in (8 bytes, u64)
        // -  [16..24]: amount_out (8 bytes, u64)
        let mut instruction_data = [UNINIT_BYTE; 24];

        // Set discriminator (8 bytes)
        let discriminator: [u8; 8] = [55, 217, 98, 86, 163, 74, 180, 173];
        write_bytes(&mut instruction_data[0..8], &discriminator);
        
        // Set max_amount_in (8 bytes)
        write_bytes(&mut instruction_data[8..16], &self.max_amount_in.to_le_bytes());
        
        // Set amount_out (8 bytes)
        write_bytes(&mut instruction_data[16..24], &self.amount_out.to_le_bytes());

        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &account_metas,
            data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 24) },
        };

        let accounts = [
            self.payer, self.authority, self.amm_config, self.pool_state,
            self.input_token_account, self.output_token_account, self.input_vault, self.output_vault,
            self.input_token_program, self.output_token_program, self.input_token_mint, self.output_token_mint,
            self.observation_state,
        ];

        invoke_signed(&instruction, &accounts, signers)
    }
} 