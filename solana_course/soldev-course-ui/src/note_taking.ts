import {
  Keypair,
  Connection,
  clusterApiUrl,
  PublicKey,
  Transaction,
  TransactionInstruction,
  sendAndConfirmRawTransaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import * as dotenv from 'dotenv';
import base58 from 'bs58';
import { getKeypairFromEnvironment } from '@solana-developers/node-helpers';
dotenv.config({ path: '../.env' });

// Load sender and receiver keypairs from environment variables
const payer = getKeypairFromEnvironment('SENDER_SECRET_KEY');
const receiver = getKeypairFromEnvironment('RECEIVER_SECRET_KEY');
const payerAddress = payer.publicKey;
const receiverAddress = receiver.publicKey;

console.log(`Payer Address: ${payerAddress.toBase58()}`);
console.log(`Receiver Address: ${receiverAddress.toBase58()}`);

// Connection to Devnet Established Successfully!
const connection = new Connection(clusterApiUrl('devnet'));
console.log('Connection to Devnet Established Successfully!');

// Airdrop via web3.js
// await connection.requestAirdrop(payer.publicKey, web3.LAMPORTS_PER_SOL*1)

// Calling Ping function using two arguments
// 1. connection || 2. payer keyPair
/** @description Inside this function, we need to:
 * create a transaction
 * create an instruction
 * add the instruction to the transaction
 * send the transaction. */
const PING_PROGRAM_ADDRESS = new PublicKey(
  'ChT1B39WKLS8qUrkLvFDXMhEJ4F1XZzwUNHUt4AU9aVa'
);
const PING_PROGRAM_DATA_ADDRESS = new PublicKey(
  'Ah9K7dQ8EHaZqcAsgBW8w37yN2eAy3koFmUn4x3CJtod'
);
async function sendPingTransaction(connection: Connection, payer: Keypair) {
  const tx = new Transaction();
  const programId = new PublicKey(PING_PROGRAM_ADDRESS);
  const pingProgramDataId = new PublicKey(PING_PROGRAM_DATA_ADDRESS);

  const instruction = new TransactionInstruction({
    keys: [
      { pubkey: pingProgramDataId, isSigner: false, isWritable: true },
      //   { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      //   { pubkey: programId, isSigner: false, isWritable: false },
    ],
    programId: programId,
    data: Buffer.from([]),
  });

  tx.add(instruction);
  const signature = await sendAndConfirmTransaction(connection, tx, [payer]);
  console.log(`Transaction Signature: ${signature}`);
}

// Calling sendPingTransaction function
sendPingTransaction(connection, payer).then(() => {
  console.log('✅ Finished!');
});

// Sample Ping Transaction Signature
// 2eiiDS1fuPGoNTy8ta54MsyaHPWrVH551xwyCChzXJqqgeWCUb91geqAiTFxKrtQnsf5aEAkzRaLqeSyYxsbcSnt
