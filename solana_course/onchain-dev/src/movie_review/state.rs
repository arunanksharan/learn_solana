use borsh::{BorshDeserialize, BorshSerialize};

// Our MovieAccountState struct will require the following parameters:

// is_initialized - shows whether or not the account has been initialized
// rating - user’s rating of the movie
// description - user’s description of the movie
// title - title of the movie the user is reviewing
// id - unique id of the movie review

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MovieAccountState {
    pub title: String,
    pub rating: u8,
    pub description: String,
    pub id: u64,
    pub is_initialised: bool,
}
