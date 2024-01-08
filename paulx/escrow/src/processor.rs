use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{error::EscrowError, instruction::EscrowInstruction, state::Escrow};
use spl_token::state::Account as TokenAccount;


pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EscrowInstruction::unpack(instruction_data)?;

        match instruction {
            EscrowInstruction::InitEscrow { amount } => {
                msg!("Instruction: InitEscrow");
                Self::process_init_escrow(accounts, amount, program_id)
            },
            EscrowInstruction::Exchange { amount } => {
                msg!("Instruction: Exchange");
                Self::process_exchange(accounts, amount, program_id)
            },
        }
    }

    fn process_init_escrow(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        /// Alice's details
        /// Initializer = Alice Main Account - owned by System Program
        /// temp_token_account = Alice Temp Account for token X created to put 10 token X into it - owned by Alice Main Account
        /// token_to_receive_account = Alice Token Y account to receive 100 token Y - owned by Alice Main Account
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let temp_token_account = next_account_info(account_info_iter)?;
        let token_to_receive_account = next_account_info(account_info_iter)?;
        if *token_to_receive_account != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        /// Escrow Account Details
        let escrow_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            return Err(EscrowError::NotRentExempt.into());
        }

        let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;
        if escrow_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        /// Escrow Account Details
        escrow_info.is_initialized = true;
        escrow_info.initializer_pubkey = *initializer.key;
        escrow_info.temp_token_account_pubkey = *temp_token_account.key;
        escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
        escrow_info.expected_amount = amount;

        /// Escrow Account Details Serialised and packed into its account
        Escrow::pack(escrow_info, &mut escrow_account.try_borrow_mut_data()?)?;

        /// PDA = Program Derived Address
        let (pda, _bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

        /// Invoke token program to transfer Alice Temporary Token X Account with 10 X tokens to PDA
        let token_program = next_account_info(account_info_iter)?;
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key, temp_token_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            initializer.key,
            &[&initializer.key],
        )
        msg!("Calling the token program to transfer token account ownership...");
        invoke(&owner_change_ix,
            &[temp_token_account.clone(), 
            initializer.clone(), 
            token_program.clone(),])?;

        Ok(())
    }

    fn process_exchange(accounts: &[AccountInfo], amount_expected_by_taker: u64, program_id: &Pubkey,) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        /// Bob's main account as signer
        let taker = next_account_info(Account_info_iter)?;

        if !taker.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        /// Bob's token account for token Y as writable
        let takers_sending_token_account = next_account_info(account_info_iter)?;

        // Bob's token account for token X as writable - used by Bob to receive 10 token X from Alice
        let takers_token_to_receive_account = next_account_info(account_info_iter)?;

        /// PDA's temp token account as writable - used by PDA to receive 10 token X from Alice
        let pdas_temp_token_account = next_account_info(account_info_iter)?;

        /// PDA's temp token account information
        let pdas_temp_token_account_info = TokenAccount::unpack(&pdas_temp_token_account.try_borrow_data()?)?;

        /// Regenerating PDA using escrow program_id
        let (pda, bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

        if amount_expected_by_taker != pdas_temp_token_account_info.amount {
            return Err(EscrowError::ExpectedAmountMismatch.into());
        }

        /// Alice - initializer of escrow's details - main account, Alice's token account for token Y
        let initializers_main_account = next_account_info(account_info_iter)?;
        let initializers_token_to_receive_account = next_account_info(account_info_iter)?;
        let escrow_account = next_account_info(account_info_iter)?;

        /// State information stored in escrow account state
        let escrow_info = Escrow::unpack(&escrow_account.try_borrow_data()?)?;

        /// Check if the PDA stored in Escrow state data to which Alice transferred 10 token X ownership is the same as the PDA provided by Bob as part of instruction
        if escrow_info.temp_token_account_pubkey != *pdas_temp_token_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        /// Check if Escrow state data Alice public key is the same as the initializer public key provided by Bob as part of instruction
        if escrow_info.initializer_pubkey != *initializers_main_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        /// Check if Escrow state data Alice token account for token Y is the same as the initializer token account for token Y provided by Bob as part of instruction
        if escrow_info.initializer_token_to_receive_account_pubkey != *initializers_token_to_receive_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        /// Address for token program
        let token_program = next_account_info(account_info_iter)?;
        
        /// Bob's instruction to transfer 100 token Y to Alice
        let transfer_to_initializer_ix = spl_token::instruction::transfer(
            token_program.key, 
            takers_sending_token_account.key, 
            initializers_token_to_receive_account.key, 
            taker.key, 
            &[&taker.key], 
            escrow_info.expected_amount);
        
        msg!("Calling the token program to transfer tokens to the escrow's initializer...");
        invoke(
            &transfer_to_initializer_ix,
            &[
                takers_sending_token_account.clone(),    // Bob's token account for token Y
                initializers_token_to_receive_account.clone(),    // Alice's token account for token Y
                taker.clone(),    // Bob's main account
                token_program.clone(),
            ]
        )?;

        /// Transfer of Token X from PDA holding it currently to Bob's token x account
        let pda_account = next_account_info(account_info_iter)?;

        let transfer_to_taker_ix = spl_token::instruction::transfer(
            token_program.key,
            pdas_temp_token_account.key,    // PDA's temp token account - holding token x from Alice
            takers_token_to_receive_account.key,    // Bob's token account for token X
            &pda,
            &[&pda],
            pdas_temp_token_account_info.amount,
        )?;

        msg!("Calling the token program to transfer tokens to the escrow's taker Bob from PDA...");
        invoke_signed(
            &transfer_to_taker_ix, 
            &[
                pdas_temp_token_account.clone(), 
                takers_token_to_receive_account.clone(), 
                pda_account.clone(),    // PDA's main account - signed property set to true by runtime
                token_program.clone(),
            ],
            &[&[&b"escrow"[..], &[bump_seed]]],
        )?;

        /// Close PDA's temp token account
        let close_pdas_temp_acc_ix = spl_token::instruction::close_account(
            token_program.key,
            pdas_temp_token_account.key,
            initializers_main_account.key,
            &pda,
            &[&pda]
        )?;
        msg!("Calling the token program to close pda's temp account...");
        invoke_signed(
            &close_pdas_temp_acc_ix,
            &[
                pdas_temp_token_account.clone(),
                initializers_main_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"escrow"[..], &[bump_seed]]],
        )?;

        msg!("Closing the escrow account...");
        **initializers_main_account.lamports.borrow_mut() = initializers_main_account.lamports()
        .checked_add(escrow_account.lamports())
        .ok_or(EscrowError::AmountOverflow)?;
        
        **escrow_account.lamports.borrow_mut() = 0;
        *escrow_account.try_borrow_mut_data()? = &mut [];
        
        Ok(())
    }

}
