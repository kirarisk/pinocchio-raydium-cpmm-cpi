use core::mem;

use pinocchio::{account_info::AccountInfo, msg, program_error::ProgramError, ProgramResult};
use pinocchio_raydium_cpmm_cpi::instructions::{SwapBaseInput, SwapBaseOutput, Deposit as DepositCpmm, Initialize as InitializeCpmm, Withdraw as WithdrawCpmm};


pub struct SwapAccounts<'a>{
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

}

impl<'a> TryFrom<&'a [AccountInfo]> for SwapAccounts<'a>{
    type Error = ProgramError;

    fn try_from(value: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, authority, amm_config, pool_state, input_token_account, output_token_account, input_vault, output_vault, input_token_program, output_token_program, input_token_mint, output_token_mint, observation_state, _] = value else{
            return Err(ProgramError::InvalidAccountData);
        };
        Ok(Self{
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
            observation_state
        })
    }

}

pub struct SwapInstructionData{
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

impl<'a> TryFrom<&'a [u8]> for SwapInstructionData{
    type Error = ProgramError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        msg!("swap data");
        if value.len()!= mem::size_of::<u64>()*2 {
            return Err(ProgramError::InvalidAccountData); 
        };

        let amount_in = <u64>::from_le_bytes(value[0..8].try_into().unwrap());
        let minimum_amount_out = <u64>::from_le_bytes(value[8..16].try_into().unwrap());






    Ok(Self { amount_in, minimum_amount_out })

    }

}

pub struct Swap<'a>{
    pub accounts: SwapAccounts<'a>,
    pub data: SwapInstructionData
}


impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Swap<'a>{
    type Error = ProgramError;


    fn try_from((dats,accs): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        msg!("swap accs and data instruction try from");
        let accounts = SwapAccounts::try_from(accs)?;
        let data = SwapInstructionData::try_from(dats)?;

        
        Ok(Self {accounts,data})
    }
}


impl<'a> Swap<'a>{
    pub const DISCRIMINATOR: &'a u8 = &2; 
    pub fn process(&mut self) -> ProgramResult{
        msg!("swap process");
        let swap_context = SwapBaseInput {
            payer: self.accounts.payer,
            authority: self.accounts.authority,
            amm_config: self.accounts.amm_config,
            pool_state: self.accounts.pool_state,
            input_token_account: self.accounts.input_token_account,
            output_token_account: self.accounts.output_token_account,
            input_vault: self.accounts.input_vault,
            output_vault: self.accounts.output_vault,
            input_token_program: self.accounts.input_token_program,
            output_token_program: self.accounts.output_token_program,
            input_token_mint: self.accounts.input_token_mint,
            output_token_mint: self.accounts.output_token_mint,
            observation_state: self.accounts.observation_state,
            amount_in: self.data.amount_in,
            minimum_amount_out: self.data.minimum_amount_out,
        };
    
        msg!("swap context done, invoking now");
        swap_context.invoke().map_err(|_| ProgramError::InvalidAccountData)?;


        Ok(())

    }
}



// =============== CPMM Initialize Instruction ===============

pub struct InitializeCPMMAccounts<'a>{
    pub creator: &'a AccountInfo,
    pub amm_config: &'a AccountInfo,
    pub authority: &'a AccountInfo,
    pub pool_state: &'a AccountInfo,
    pub token_0_mint: &'a AccountInfo,
    pub token_1_mint: &'a AccountInfo,
    pub lp_mint: &'a AccountInfo,
    pub creator_token_0: &'a AccountInfo,
    pub creator_token_1: &'a AccountInfo,
    pub creator_lp_token: &'a AccountInfo,
    pub token_0_vault: &'a AccountInfo,
    pub token_1_vault: &'a AccountInfo,
    pub create_pool_fee: &'a AccountInfo,
    pub observation_state: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub token_0_program: &'a AccountInfo,
    pub token_1_program: &'a AccountInfo,
    pub associated_token_program: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub rent: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for InitializeCPMMAccounts<'a>{
    type Error = ProgramError;

    fn try_from(value: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [creator, amm_config, authority, pool_state, token_0_mint, token_1_mint, lp_mint, creator_token_0, creator_token_1, creator_lp_token, token_0_vault, token_1_vault, create_pool_fee, observation_state, token_program, token_0_program, token_1_program, associated_token_program, system_program, rent, _] = value else{
            return Err(ProgramError::InvalidAccountData);
        };
        Ok(Self{
            creator, amm_config, authority, pool_state, token_0_mint, token_1_mint, lp_mint,
            creator_token_0, creator_token_1, creator_lp_token, token_0_vault, token_1_vault,
            create_pool_fee, observation_state, token_program, token_0_program, token_1_program,
            associated_token_program, system_program, rent
        })
    }
}

pub struct InitializeCPMMInstructionData{
    pub init_amount_0: u64,
    pub init_amount_1: u64,
    pub open_time: u64,
}

impl<'a> TryFrom<&'a [u8]> for InitializeCPMMInstructionData{
    type Error = ProgramError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        msg!("initialize cpmm data");
        if value.len()!= mem::size_of::<u64>()*3 {
            return Err(ProgramError::InvalidAccountData); 
        };

        let init_amount_0 = <u64>::from_le_bytes(value[0..8].try_into().unwrap());
        let init_amount_1 = <u64>::from_le_bytes(value[8..16].try_into().unwrap());
        let open_time = <u64>::from_le_bytes(value[16..24].try_into().unwrap());

        Ok(Self { init_amount_0, init_amount_1, open_time })
    }
}

pub struct InitializeCPMM<'a>{
    pub accounts: InitializeCPMMAccounts<'a>,
    pub data: InitializeCPMMInstructionData
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for InitializeCPMM<'a>{
    type Error = ProgramError;

    fn try_from((dats,accs): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        msg!("initialize cpmm accs and data instruction try from");
        let accounts = InitializeCPMMAccounts::try_from(accs)?;
        let data = InitializeCPMMInstructionData::try_from(dats)?;
        
        Ok(Self {accounts,data})
    }
}

impl<'a> InitializeCPMM<'a>{
    pub const DISCRIMINATOR: &'a u8 = &0; 
    pub fn process(&mut self) -> ProgramResult{
        msg!("initialize cpmm process");
        let initialize_context = InitializeCpmm {
            creator: self.accounts.creator,
            amm_config: self.accounts.amm_config,
            authority: self.accounts.authority,
            pool_state: self.accounts.pool_state,
            token_0_mint: self.accounts.token_0_mint,
            token_1_mint: self.accounts.token_1_mint,
            lp_mint: self.accounts.lp_mint,
            creator_token_0: self.accounts.creator_token_0,
            creator_token_1: self.accounts.creator_token_1,
            creator_lp_token: self.accounts.creator_lp_token,
            token_0_vault: self.accounts.token_0_vault,
            token_1_vault: self.accounts.token_1_vault,
            create_pool_fee: self.accounts.create_pool_fee,
            observation_state: self.accounts.observation_state,
            token_program: self.accounts.token_program,
            token_0_program: self.accounts.token_0_program,
            token_1_program: self.accounts.token_1_program,
            associated_token_program: self.accounts.associated_token_program,
            system_program: self.accounts.system_program,
            rent: self.accounts.rent,
            init_amount_0: self.data.init_amount_0,
            init_amount_1: self.data.init_amount_1,
            open_time: self.data.open_time,
        };
    
        msg!("initialize cpmm context done, invoking now");
        initialize_context.invoke().map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(())
    }
}

// =============== CPMM Deposit Instruction ===============

pub struct DepositCPMMAccounts<'a>{
    pub owner: &'a AccountInfo,
    pub authority: &'a AccountInfo,
    pub pool_state: &'a AccountInfo,
    pub owner_lp_token: &'a AccountInfo,
    pub token_0_account: &'a AccountInfo,
    pub token_1_account: &'a AccountInfo,
    pub token_0_vault: &'a AccountInfo,
    pub token_1_vault: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub token_program_2022: &'a AccountInfo,
    pub vault_0_mint: &'a AccountInfo,
    pub vault_1_mint: &'a AccountInfo,
    pub lp_mint: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for DepositCPMMAccounts<'a>{
    type Error = ProgramError;

    fn try_from(value: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [owner, authority, pool_state, owner_lp_token, token_0_account, token_1_account, token_0_vault, token_1_vault, token_program, token_program_2022, vault_0_mint, vault_1_mint, lp_mint, _] = value else{
            return Err(ProgramError::InvalidAccountData);
        };
        Ok(Self{
            owner, authority, pool_state, owner_lp_token, token_0_account, token_1_account,
            token_0_vault, token_1_vault, token_program, token_program_2022, vault_0_mint, vault_1_mint, lp_mint
        })
    }
}

pub struct DepositCPMMInstructionData{
    pub lp_token_amount: u64,
    pub maximum_token_0_amount: u64,
    pub maximum_token_1_amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for DepositCPMMInstructionData{
    type Error = ProgramError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        msg!("deposit cpmm data");
        if value.len()!= mem::size_of::<u64>()*3 {
            return Err(ProgramError::InvalidAccountData); 
        };

        let lp_token_amount = <u64>::from_le_bytes(value[0..8].try_into().unwrap());
        let maximum_token_0_amount = <u64>::from_le_bytes(value[8..16].try_into().unwrap());
        let maximum_token_1_amount = <u64>::from_le_bytes(value[16..24].try_into().unwrap());

        Ok(Self { lp_token_amount, maximum_token_0_amount, maximum_token_1_amount })
    }
}

pub struct DepositCPMM<'a>{
    pub accounts: DepositCPMMAccounts<'a>,
    pub data: DepositCPMMInstructionData
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for DepositCPMM<'a>{
    type Error = ProgramError;

    fn try_from((dats,accs): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        msg!("deposit cpmm accs and data instruction try from");
        let accounts = DepositCPMMAccounts::try_from(accs)?;
        let data = DepositCPMMInstructionData::try_from(dats)?;
        
        Ok(Self {accounts,data})
    }
}

impl<'a> DepositCPMM<'a>{
    pub const DISCRIMINATOR: &'a u8 = &1; 
    pub fn process(&mut self) -> ProgramResult{
        msg!("deposit cpmm process");
        let deposit_context = DepositCpmm {
            owner: self.accounts.owner,
            authority: self.accounts.authority,
            pool_state: self.accounts.pool_state,
            owner_lp_token: self.accounts.owner_lp_token,
            token_0_account: self.accounts.token_0_account,
            token_1_account: self.accounts.token_1_account,
            token_0_vault: self.accounts.token_0_vault,
            token_1_vault: self.accounts.token_1_vault,
            token_program: self.accounts.token_program,
            token_program_2022: self.accounts.token_program_2022,
            vault_0_mint: self.accounts.vault_0_mint,
            vault_1_mint: self.accounts.vault_1_mint,
            lp_mint: self.accounts.lp_mint,
            lp_token_amount: self.data.lp_token_amount,
            maximum_token_0_amount: self.data.maximum_token_0_amount,
            maximum_token_1_amount: self.data.maximum_token_1_amount,
        };
    
        msg!("deposit cpmm context done, invoking now");
        deposit_context.invoke().map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(())
    }
}

// =============== CPMM SwapBaseOutput Instruction ===============

pub struct SwapBaseOutputAccounts<'a>{
    pub payer: &'a AccountInfo,
    pub authority: &'a AccountInfo,
    pub amm_config: &'a AccountInfo,
    pub pool_state: &'a AccountInfo,
    pub input_token_account: &'a AccountInfo,
    pub output_token_account: &'a AccountInfo,
    pub input_vault: &'a AccountInfo,
    pub output_vault: &'a AccountInfo,
    pub input_token_program: &'a AccountInfo,
    pub output_token_program: &'a AccountInfo,
    pub input_token_mint: &'a AccountInfo,
    pub output_token_mint: &'a AccountInfo,
    pub observation_state: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for SwapBaseOutputAccounts<'a>{
    type Error = ProgramError;

    fn try_from(value: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, authority, amm_config, pool_state, input_token_account, output_token_account, input_vault, output_vault, input_token_program, output_token_program, input_token_mint, output_token_mint, observation_state, _] = value else{
            return Err(ProgramError::InvalidAccountData);
        };
        Ok(Self{
            payer, authority, amm_config, pool_state, input_token_account, output_token_account,
            input_vault, output_vault, input_token_program, output_token_program, input_token_mint,
            output_token_mint, observation_state
        })
    }
}

pub struct SwapBaseOutputInstructionData{
    pub max_amount_in: u64,
    pub amount_out: u64,
}

impl<'a> TryFrom<&'a [u8]> for SwapBaseOutputInstructionData{
    type Error = ProgramError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        msg!("swap base output data");
        if value.len()!= mem::size_of::<u64>()*2 {
            return Err(ProgramError::InvalidAccountData); 
        };

        let max_amount_in = <u64>::from_le_bytes(value[0..8].try_into().unwrap());
        let amount_out = <u64>::from_le_bytes(value[8..16].try_into().unwrap());

        Ok(Self { max_amount_in, amount_out })
    }
}

pub struct SwapBaseOutputCPMM<'a>{
    pub accounts: SwapBaseOutputAccounts<'a>,
    pub data: SwapBaseOutputInstructionData
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for SwapBaseOutputCPMM<'a>{
    type Error = ProgramError;

    fn try_from((dats,accs): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        msg!("swap base output accs and data instruction try from");
        let accounts = SwapBaseOutputAccounts::try_from(accs)?;
        let data = SwapBaseOutputInstructionData::try_from(dats)?;
        
        Ok(Self {accounts,data})
    }
}

impl<'a> SwapBaseOutputCPMM<'a>{
    pub const DISCRIMINATOR: &'a u8 = &3; 
    pub fn process(&mut self) -> ProgramResult{
        msg!("swap base output process");
        let swap_context = SwapBaseOutput {
            payer: self.accounts.payer,
            authority: self.accounts.authority,
            amm_config: self.accounts.amm_config,
            pool_state: self.accounts.pool_state,
            input_token_account: self.accounts.input_token_account,
            output_token_account: self.accounts.output_token_account,
            input_vault: self.accounts.input_vault,
            output_vault: self.accounts.output_vault,
            input_token_program: self.accounts.input_token_program,
            output_token_program: self.accounts.output_token_program,
            input_token_mint: self.accounts.input_token_mint,
            output_token_mint: self.accounts.output_token_mint,
            observation_state: self.accounts.observation_state,
            max_amount_in: self.data.max_amount_in,
            amount_out: self.data.amount_out,
        };
    
        msg!("swap base output context done, invoking now");
        swap_context.invoke().map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(())
    }
}

// =============== CPMM Withdraw Instruction ===============

pub struct WithdrawCPMMAccounts<'a>{
    pub owner: &'a AccountInfo,
    pub authority: &'a AccountInfo,
    pub pool_state: &'a AccountInfo,
    pub owner_lp_token: &'a AccountInfo,
    pub token_0_account: &'a AccountInfo,
    pub token_1_account: &'a AccountInfo,
    pub token_0_vault: &'a AccountInfo,
    pub token_1_vault: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub token_program_2022: &'a AccountInfo,
    pub vault_0_mint: &'a AccountInfo,
    pub vault_1_mint: &'a AccountInfo,
    pub lp_mint: &'a AccountInfo,
    pub memo_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for WithdrawCPMMAccounts<'a>{
    type Error = ProgramError;

    fn try_from(value: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [owner, authority, pool_state, owner_lp_token, token_0_account, token_1_account, token_0_vault, token_1_vault, token_program, token_program_2022, vault_0_mint, vault_1_mint, lp_mint, memo_program, _] = value else{
            return Err(ProgramError::InvalidAccountData);
        };
        Ok(Self{
            owner, authority, pool_state, owner_lp_token, token_0_account, token_1_account,
            token_0_vault, token_1_vault, token_program, token_program_2022, vault_0_mint, vault_1_mint,
            lp_mint, memo_program
        })
    }
}

pub struct WithdrawCPMMInstructionData{
    pub lp_token_amount: u64,
    pub minimum_token_0_amount: u64,
    pub minimum_token_1_amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for WithdrawCPMMInstructionData{
    type Error = ProgramError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        msg!("withdraw cpmm data");
        if value.len()!= mem::size_of::<u64>()*3 {
            return Err(ProgramError::InvalidAccountData); 
        };

        let lp_token_amount = <u64>::from_le_bytes(value[0..8].try_into().unwrap());
        let minimum_token_0_amount = <u64>::from_le_bytes(value[8..16].try_into().unwrap());
        let minimum_token_1_amount = <u64>::from_le_bytes(value[16..24].try_into().unwrap());

        Ok(Self { lp_token_amount, minimum_token_0_amount, minimum_token_1_amount })
    }
}

pub struct WithdrawCPMM<'a>{
    pub accounts: WithdrawCPMMAccounts<'a>,
    pub data: WithdrawCPMMInstructionData
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for WithdrawCPMM<'a>{
    type Error = ProgramError;

    fn try_from((dats,accs): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        msg!("withdraw cpmm accs and data instruction try from");
        let accounts = WithdrawCPMMAccounts::try_from(accs)?;
        let data = WithdrawCPMMInstructionData::try_from(dats)?;
        
        Ok(Self {accounts,data})
    }
}

impl<'a> WithdrawCPMM<'a>{
    pub const DISCRIMINATOR: &'a u8 = &4; 
    pub fn process(&mut self) -> ProgramResult{
        msg!("withdraw cpmm process");
        let withdraw_context = WithdrawCpmm {
            owner: self.accounts.owner,
            authority: self.accounts.authority,
            pool_state: self.accounts.pool_state,
            owner_lp_token: self.accounts.owner_lp_token,
            token_0_account: self.accounts.token_0_account,
            token_1_account: self.accounts.token_1_account,
            token_0_vault: self.accounts.token_0_vault,
            token_1_vault: self.accounts.token_1_vault,
            token_program: self.accounts.token_program,
            token_program_2022: self.accounts.token_program_2022,
            vault_0_mint: self.accounts.vault_0_mint,
            vault_1_mint: self.accounts.vault_1_mint,
            lp_mint: self.accounts.lp_mint,
            memo_program: self.accounts.memo_program,
            lp_token_amount: self.data.lp_token_amount,
            minimum_token_0_amount: self.data.minimum_token_0_amount,
            minimum_token_1_amount: self.data.minimum_token_1_amount,
        };
    
        msg!("withdraw cpmm context done, invoking now");
        withdraw_context.invoke().map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(())
    }
}

