use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

// MovieReview enum: provides an enum of possible CRUD operations and data required for them
pub enum MovieInstruction {
    AddMovieReview {
        title: String,
        rating: u8,
        description: String,
        id: u64,
    },
    UpdateMovieReview {
        title: String,
        rating: u8,
        description: String,
        id: u64,
    },
    DeleteMovieReview {
        id: u64,
    },
}

// MovieReviewpayload: intermediary type for deserialization
#[derive(BorshDeserialize, BorshSerialize)]
struct MovieReviewPayload {
    title: String,
    rating: u8,
    description: String,
    id: u64,
}

impl MovieInstruction {
    // Unpack inbound buffer to associated instruction - add, update, delete
    // Input's expected format is Borsh serialised vector
    pub fn unpack(input: &[u8]) -> Result(Self, ProgramError) {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        // Desrialize instruction rest byte data into payload struct
        let payload = MovieReviewPayload::try_from_slice(rest).unwrap();

        // Match the variant/first byte and send the relevant enum choic from MovieInstruction enum - add/update/delete

        Ok(match variant {
            0 => Self::AddMovieReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
                id: payload.id,
            },
            1 => Self::UpdateMovieReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
                id: payload.id,
            },
            2 => Self::DeleteMovieReview { id: payload.id },
            _ => Err(ProgramError::InvalidInstructionData),
        })
    }
}
