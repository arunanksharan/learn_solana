use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let note_creator = next_account_info(account_info_iter)?;
    let note_pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Call unpack to deserialize instruction_data
    let instruction = NoteInstruction::unpack(instruction_data)?;
    // Match the returned data struct to what you expect
    match instruction {
        NoteInstruction::CreateNote { title, body, id } => {
            // Write code to create a note
        }
        NoteInstruction::UpdateNote { title, body, id } => {
            // Write code to update a note
        }
        NoteInstruction::DeleteNote { id } => {
            // Write code to delete a note
        }
    }
}
