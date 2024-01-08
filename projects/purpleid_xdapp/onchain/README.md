## Scope fo the project:

### User story
1. As a user, I can check if a particular namespace 'abc.sol' is available.

2. As a user with wallet address 'xyz', I can claim an available namespace of my choice.

3. As a user, I can delete any associated namespace.

### Namespace Struct
pub struct CrosschainID {
    pub owner: Pubkey, // 32 bytes
    pub namespace: Vec<u8>, // 32 bytes
}