# Escrow
Imagine Alice has an asset A and Bob has an asset B. They would like to trade their assets but neither wants to send their asset first. After all, what if the other party does not hold up their end of the trade and runs away with both assets? A deadlock will be reached where no party wants to send their asset first.

The traditional way to solve this problem is to introduce a third party C which both A and B trust. A or B can now go first and send their asset to C. C then waits for the other party to send their asset and only then does C release both assets.

## Project Structure
.
├─ src
│  ├─ lib.rs -> registering modules
│  ├─ entrypoint.rs -> entrypoint to the program
│  ├─ instruction.rs -> program API, (de)serializing instruction data
│  ├─ processor.rs -> program logic
│  ├─ state.rs -> program objects, (de)serializing state
│  ├─ error.rs -> program specific errors
├─ .gitignore
├─ Cargo.lock
├─ Cargo.toml
├─ Xargo.toml

- Entrypoint is called with program_id, accounts, instruction_data
- Entrypoint forwards the information via process_instruction to processor
- Processor aks instruction.rs to decode instruction_data
- Using decoded data, processor now decides which function to use to process request
- Processor may use state.rs to encode state into or decode state of an account passed into entrypoint
- While there is only one entrypoint, program execution can follow different paths depending on the given instruction data that is decoded inside instruction.rs