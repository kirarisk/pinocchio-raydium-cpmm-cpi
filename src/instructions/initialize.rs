use core::slice::from_raw_parts;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};

use crate::{write_bytes, UNINIT_BYTE};

/// Creates a pool for the given token pair and the initial price
///
/// ### Accounts:
///   0. `[WRITE, SIGNER]` creator - Address paying to create the pool
///   1. `[]` amm_config - Which config the pool belongs to
///   2. `[]` authority - Pool vault and lp mint authority (PDA)
///   3. `[WRITE]` pool_state - Initialize an account to store the pool state
///   4. `[]` token_0_mint - Token_0 mint, key must be smaller than token_1 mint
///   5. `[]` token_1_mint - Token_1 mint, key must be greater than token_0 mint
///   6. `[WRITE]` lp_mint - Pool lp mint (PDA)
///   7. `[WRITE]` creator_token_0 - Creator token0 account
///   8. `[WRITE]` creator_token_1 - Creator token1 account
///   9. `[WRITE]` creator_lp_token - Creator lp token account
///   10. `[WRITE]` token_0_vault - Token_0 vault for the pool (PDA)
///   11. `[WRITE]` token_1_vault - Token_1 vault for the pool (PDA)
///   12. `[WRITE]` create_pool_fee - Create pool fee account
///   13. `[WRITE]` observation_state - Account to store oracle observations (PDA)
///   14. `[]` token_program - Program to create mint account and mint tokens
///   15. `[]` token_0_program - Spl token program or token program 2022
///   16. `[]` token_1_program - Spl token program or token program 2022
///   17. `[]` associated_token_program - Program to create an ATA for receiving position NFT
///   18. `[]` system_program - To create a new program account
///   19. `[]` rent - Sysvar for program account
pub struct Initialize<'a> {
    /// Address paying to create the pool. Can be anyone
    pub creator: &'a AccountInfo,
    /// Which config the pool belongs to.
    pub amm_config: &'a AccountInfo,
    /// CHECK: pool vault and lp mint authority
    pub authority: &'a AccountInfo,
    /// CHECK: Initialize an account to store the pool state
    pub pool_state: &'a AccountInfo,
    /// Token_0 mint, the key must smaller then token_1 mint.
    pub token_0_mint: &'a AccountInfo,
    /// Token_1 mint, the key must grater then token_0 mint.
    pub token_1_mint: &'a AccountInfo,
    /// pool lp mint
    pub lp_mint: &'a AccountInfo,
    /// payer token0 account
    pub creator_token_0: &'a AccountInfo,
    /// creator token1 account
    pub creator_token_1: &'a AccountInfo,
    /// creator lp token account
    pub creator_lp_token: &'a AccountInfo,
    /// Token_0 vault for the pool
    pub token_0_vault: &'a AccountInfo,
    /// Token_1 vault for the pool
    pub token_1_vault: &'a AccountInfo,
    /// create pool fee account
    pub create_pool_fee: &'a AccountInfo,
    /// an account to store oracle observations
    pub observation_state: &'a AccountInfo,
    /// Program to create mint account and mint tokens
    pub token_program: &'a AccountInfo,
    /// Spl token program or token program 2022
    pub token_0_program: &'a AccountInfo,
    /// Spl token program or token program 2022
    pub token_1_program: &'a AccountInfo,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: &'a AccountInfo,
    /// To create a new program account
    pub system_program: &'a AccountInfo,
    /// Sysvar for program account
    pub rent: &'a AccountInfo,
    
    /// Instruction parameters
    pub init_amount_0: u64,
    pub init_amount_1: u64,
    pub open_time: u64,
}

impl Initialize<'_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        // Account metadata - 20 accounts total
        let account_metas: [AccountMeta; 20] = [
            AccountMeta::writable_signer(self.creator.key()),
            AccountMeta::readonly(self.amm_config.key()),
            AccountMeta::readonly(self.authority.key()),
            AccountMeta::writable(self.pool_state.key()),
            AccountMeta::readonly(self.token_0_mint.key()),
            AccountMeta::readonly(self.token_1_mint.key()),
            AccountMeta::writable(self.lp_mint.key()),
            AccountMeta::writable(self.creator_token_0.key()),
            AccountMeta::writable(self.creator_token_1.key()),
            AccountMeta::writable(self.creator_lp_token.key()),
            AccountMeta::writable(self.token_0_vault.key()),
            AccountMeta::writable(self.token_1_vault.key()),
            AccountMeta::writable(self.create_pool_fee.key()),
            AccountMeta::writable(self.observation_state.key()),
            AccountMeta::readonly(self.token_program.key()),
            AccountMeta::readonly(self.token_0_program.key()),
            AccountMeta::readonly(self.token_1_program.key()),
            AccountMeta::readonly(self.associated_token_program.key()),
            AccountMeta::readonly(self.system_program.key()),
            AccountMeta::readonly(self.rent.key()),
        ];

        // Instruction data layout:
        // -  [0..8]: instruction discriminator (8 bytes)
        // -  [8..16]: init_amount_0 (8 bytes, u64)
        // -  [16..24]: init_amount_1 (8 bytes, u64)
        // -  [24..32]: open_time (8 bytes, u64)
        let mut instruction_data = [UNINIT_BYTE; 32];

        // Set discriminator (8 bytes)
        let discriminator: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
        write_bytes(&mut instruction_data[0..8], &discriminator);
        
        // Set init_amount_0 (8 bytes)
        write_bytes(&mut instruction_data[8..16], &self.init_amount_0.to_le_bytes());
        
        // Set init_amount_1 (8 bytes)
        write_bytes(&mut instruction_data[16..24], &self.init_amount_1.to_le_bytes());
        
        // Set open_time (8 bytes)
        write_bytes(&mut instruction_data[24..32], &self.open_time.to_le_bytes());

        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &account_metas,
            data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 32) },
        };

        let accounts = [
            self.creator, self.amm_config, self.authority, self.pool_state,
            self.token_0_mint, self.token_1_mint, self.lp_mint, self.creator_token_0,
            self.creator_token_1, self.creator_lp_token, self.token_0_vault, self.token_1_vault,
            self.create_pool_fee, self.observation_state, self.token_program, self.token_0_program,
            self.token_1_program, self.associated_token_program, self.system_program, self.rent,
        ];

        invoke_signed(&instruction, &accounts, signers)
    }
} 