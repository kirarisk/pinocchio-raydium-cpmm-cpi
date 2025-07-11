use core::slice::from_raw_parts;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};

use crate::{write_bytes, UNINIT_BYTE};

/// Deposits liquidity into the pool
///
/// ### Accounts:
///   0. `[SIGNER]` owner - Pays to mint the position
///   1. `[]` authority - Pool vault and lp mint authority (PDA)
///   2. `[WRITE]` pool_state - Pool state account
///   3. `[WRITE]` owner_lp_token - Owner lp token account
///   4. `[WRITE]` token_0_account - The payer's token account for token_0
///   5. `[WRITE]` token_1_account - The payer's token account for token_1
///   6. `[WRITE]` token_0_vault - The address that holds pool tokens for token_0
///   7. `[WRITE]` token_1_vault - The address that holds pool tokens for token_1
///   8. `[]` token_program - token Program
///   9. `[]` token_program_2022 - Token program 2022
///   10. `[]` vault_0_mint - The mint of token_0 vault
///   11. `[]` vault_1_mint - The mint of token_1 vault
///   12. `[WRITE]` lp_mint - Lp token mint
pub struct Deposit<'a> {
    /// Pays to mint the position
    pub owner: &'a AccountInfo,
    /// CHECK: pool vault and lp mint authority
    pub authority: &'a AccountInfo,
    /// Pool state account
    pub pool_state: &'a AccountInfo,
    /// Owner lp token account
    pub owner_lp_token: &'a AccountInfo,
    /// The payer's token account for token_0
    pub token_0_account: &'a AccountInfo,
    /// The payer's token account for token_1
    pub token_1_account: &'a AccountInfo,
    /// The address that holds pool tokens for token_0
    pub token_0_vault: &'a AccountInfo,
    /// The address that holds pool tokens for token_1
    pub token_1_vault: &'a AccountInfo,
    /// token Program
    pub token_program: &'a AccountInfo,
    /// Token program 2022
    pub token_program_2022: &'a AccountInfo,
    /// The mint of token_0 vault
    pub vault_0_mint: &'a AccountInfo,
    /// The mint of token_1 vault
    pub vault_1_mint: &'a AccountInfo,
    /// Lp token mint
    pub lp_mint: &'a AccountInfo,
    
    /// Instruction parameters
    pub lp_token_amount: u64,
    pub maximum_token_0_amount: u64,
    pub maximum_token_1_amount: u64,
}

impl Deposit<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        // Account metadata - 13 accounts total
        let account_metas: [AccountMeta; 13] = [
            AccountMeta::readonly_signer(self.owner.key()),
            AccountMeta::readonly(self.authority.key()),
            AccountMeta::writable(self.pool_state.key()),
            AccountMeta::writable(self.owner_lp_token.key()),
            AccountMeta::writable(self.token_0_account.key()),
            AccountMeta::writable(self.token_1_account.key()),
            AccountMeta::writable(self.token_0_vault.key()),
            AccountMeta::writable(self.token_1_vault.key()),
            AccountMeta::readonly(self.token_program.key()),
            AccountMeta::readonly(self.token_program_2022.key()),
            AccountMeta::readonly(self.vault_0_mint.key()),
            AccountMeta::readonly(self.vault_1_mint.key()),
            AccountMeta::writable(self.lp_mint.key()),
        ];

        // Instruction data layout:
        // -  [0..8]: instruction discriminator (8 bytes)
        // -  [8..16]: lp_token_amount (8 bytes, u64)
        // -  [16..24]: maximum_token_0_amount (8 bytes, u64)
        // -  [24..32]: maximum_token_1_amount (8 bytes, u64)
        let mut instruction_data = [UNINIT_BYTE; 32];

        // Set discriminator (8 bytes)
        let discriminator: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
        write_bytes(&mut instruction_data[0..8], &discriminator);
        
        // Set lp_token_amount (8 bytes)
        write_bytes(&mut instruction_data[8..16], &self.lp_token_amount.to_le_bytes());
        
        // Set maximum_token_0_amount (8 bytes)
        write_bytes(&mut instruction_data[16..24], &self.maximum_token_0_amount.to_le_bytes());
        
        // Set maximum_token_1_amount (8 bytes)
        write_bytes(&mut instruction_data[24..32], &self.maximum_token_1_amount.to_le_bytes());

        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &account_metas,
            data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 32) },
        };

        let accounts = [
            self.owner, self.authority, self.pool_state, self.owner_lp_token,
            self.token_0_account, self.token_1_account, self.token_0_vault, self.token_1_vault,
            self.token_program, self.token_program_2022, self.vault_0_mint, self.vault_1_mint,
            self.lp_mint,
        ];

        invoke_signed(&instruction, &accounts, signers)
    }
} 