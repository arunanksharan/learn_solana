use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

// ns = Namespace
// NamespaceInstruction enum: provides an enum of possible CRUD operations and data required for them
pub enum NamespaceInstruction {
    AddNamespace {
        nsOwner: Pubkey,
        nsClaimed: String,
        nsLength: u8,
    },
    UpdateNamespace {
        nsOwner: Pubkey,
        nsClaimed: String,
        nsLength: u8,
    },
    DeleteNamespace {
        nsOwner: Pubkey,
        nsClaimed: String,
        nsLength: u8,
    },
}

// Namespacepayload: intermediary type for deserialization
#[derive(BorshDeserialize, BorshSerialize)]
struct NamespacePayload {
    nsOwner: Pubkey,
    nsClaimed: String,
    nsLength: u8,
}

impl NamespaceInstruction {
    // Unpack inbound buffer to associated instruction - add, update, delete
    // Input's expected format is Borsh serialised vector
    pub fn unpack(input: &[u8]) -> Result(Self, ProgramError) {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        // Desrialize instruction rest byte data into payload struct
        let payload = NamespacePayload::try_from_slice(rest).unwrap();

        // Match the variant/first byte and send the relevant enum choic from NamespaceInstruction enum - add/update/delete

        Ok(match variant {
            0 => Self::AddNamespace {
                nsOwner: payload.nsOwner,
                nsClaimed: payload.nsClaimed,
                nsLength: payload.nsLength,
            },
            1 => Self::UpdateNamespace {
                nsOwner: payload.nsOwner,
                nsClaimed: payload.nsClaimed,
                nsLength: payload.nsLength,
            },
            2 => Self::DeleteNamespace {
                nsOwner: payload.nsOwner,
                nsClaimed: payload.nsClaimed,
                nsLength: payload.nsLength,
            },
            _ => Err(ProgramError::InvalidInstructionData),
        })
    }
}

// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
// This section below may need to be moved into state.rs file
// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
#[derive(BorshSerialize, BorshDeserialize)]
struct NamespaceState {
    nsOwner: Pubkey,
    nsClaimed: String,
    nsLength: u8,
}
// To create a new account within our program we must:
// 1. Calculate the space and rent required for the account - To calculate the size the account needs to be, you would simply add up the size required to store the data in each field.
// nsOwner = pubkey = 32 bytes
// nsClaimed = String = string.len() + 4 bytes
// Length field for nsClaimed = 4 bytes
// nsLength = u8 = 1 byte
let account_len: usize = 32 + (nsClaimed.len() + 4) + 1;
// Calculate rent required
let rent = Rent::get()?;
let rent_lamports = rent.minimum_balance(account_len);

// 2. Have an address to assign the new account
// 3. Invoke the system program to create the new account
