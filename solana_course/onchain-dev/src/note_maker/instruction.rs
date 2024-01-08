enum NoteInstruction {
    CreateNote {
        title: String,
        body: String,
        id: u64,
    },
    UpdateNote {
        title: String,
        body: String,
        id: u64,
    },
    DeleteNote {
        id: u64,
    },
}

#[derive(BorshDeserialize)]
struct NoteInstructionPayload {
    id: u64,
    title: String,
    body: String,
}

impl NoteInstruction {
    // Unpack inbound buffer to associated Instruction
    // The expected format for input is a Borsh serialized vector

    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Take the first byte as the variant to
        // determine which instruction to execute
        let (&variant, rest): (&u8, &[u8]) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        // Use the temporary payload struct to deserialize
        let payload = NoteInstructionPayload::try_from_slice(rest).unwrap();

        // Match the variant to determine which data struct is expected by
        // the function and return the TestStruct or an error

        Ok(match variant {
            0 => Self::CreateNote {
                title: payload.title,
                body: payload.body,
                id: payload.id,
            },
            1 => Self::UpdateNote {
                title: payload.title,
                body: payload.body,
                id: payload.id,
            },
            2 => Self::DeleteNote { id: payload.id },
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
struct NoteState {
    title: String,
    body: String,
    id: u64,
}

// Calculate account size required for struct NoteState
let account_len: usize = (4 + title.len()) + (4 + body.len()) + 8;

// Calculate rent required
let rent = Rent::get()?;
let rent_lamports = rent.minimum_balance(account_len);

// Finding PDA
let (note_pda_account, bump_seed) = Pubkey::find_program_address(&[note_creator.key.as_ref(), id.as_bytes().as_ref(),], program_id,);

// CPI is when one program invokes an instruction on another program. To create a new account within our program, we will invoke the create_account instruction on the system program.
// pub fn invoke(instruction: &Instruction,
// account_infos: &[AccountInfo<'_>]) -> ProgramResult{}

pub fn invoke_signed(
// instruction
    instruction: &system_instruction::create_account(note_creator.key, note_pda_account.key, rent_lamports, account_len.try_into().unwrap(), program_id),

// account_infos
    account_infos: &[note_creator.clone(), note_pda_account.clone(), system_program.clone()],

// signers
    signers_seeds: &[&[note_creator.key.as_ref(), id.as_bytes().as_ref(), &[bump_seed]]],
)?;


// Borrow account data - access without taking ownership + deserialize into NoteState struct
// Update using dot notation
// serialize NoteState struct back into account data
let mut account_data = try_from_slice_unchecked::<NoteState>(note_pda_account.data.borrow()).unwrap();
account_data.title = title;
account_data.body = body;
account_data.id = id;

// Serialize NoteState struct back into account data - pass in a mutable reference to the account data
account_data.serialize(&mut &mut note_pda_account.data.borrow_mut()[..])?;
