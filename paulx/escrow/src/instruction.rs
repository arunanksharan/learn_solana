use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use create::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {
    /// Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
    /// Accounts expected:
    /// 0. `[signer]` The account of the person initializing the escrow - Alice Main Account
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer - Alice Temp Account for token X created to put 10 token X into it
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through - Alice Token Y account to receive 100 token Y
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    InitEscrow {
        /// The amount party A expects to receive of token Y
        /// Alice wants to receive 100 token Y in exchange for 10 token X to Bob
        amount: u64,
    },
    /// Accepts a trade
    /// Accounts expected:
    /// 0. `[signer]` The account of the person taking the trade - Bob Main Account
    /// 1. `[writable]` The taker's token account for the token they send - Bob sends Token Y which Alice receives
    /// 2. `[writable]` The taker's token account for the token they will receive should the trade go through - Bob's Token Account for token X - the ownership of PDA holding token X from Alice currently under Escrow PDA to be transferred to Bob
    /// 3. `[writable]` The PDA's temp token account to get tokens from and eventually close - Escrow PDA holding token x from Alice
    /// 4. `[writable]` The initializer's main account to send their rent fees to - Alice main account
    /// 5. `[writable]` The initializer's token account that will receive tokens - Alice Token Y account to receive 100 token Y
    /// 6. `[writable]` The escrow account holding the escrow info - Escrow Account - state information
    /// 7. `[]` The token program
    /// 8. `[]` The PDA account - Escrow PDA
    Exchange {
        /// Amount Bob expects to receive from Alice in terms of token x
        amount: u64,
    },
}

impl EscrowInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction]
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Exchange {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}

fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
    let amount = input
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(InvalidInstruction)?;

    Ok(amount)
}
