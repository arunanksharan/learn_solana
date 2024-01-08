use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};
pub mod instruction;
use instruction::NamespaceInstruction;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello World");

    // receive input data - call unpack to deserialize instruction_data and identiyf relevant variant - add/update/delete

    let instruction = NamespaceInstruction::unpack(instruction_data)?;

    // call associated function by matching the variant received from unpack and the MovieInstruction enum - add/update/delete and pass instruction data into it

    match instruction {
        NamespaceInstruction::AddNamespace {
            nsOwner,
            nsClaimed,
            nsLength,
        } => {
            // Make a call to `add_namespace` function
            add_namespace(program_id, accounts, nsOwner, nsClaimed, nsLength)
        }
    }
}

pub fn add_namespace(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    nsOwner: Pubkey,
    nsClaimed: String,
    nsLength: u8,
) -> ProgramResult {
    // Logging instruction data that was passed in
    msg!("Adding namespace...");
    msg!("nsOwner: {}", nsOwner);
    msg!("nsClaimed: {}", nsClaimed);
    msg!("nsLength: {}", nsLength);

    Ok(()
)