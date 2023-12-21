import {
  Connection,
  PublicKey,
  clusterApiUrl,
  LAMPORTS_PER_SOL,
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import * as dotenv from 'dotenv';
import { getKeypairFromEnvironment } from '@solana-developers/node-helpers';

dotenv.config({ path: '../.env' });

const connection = new Connection(clusterApiUrl('devnet'));
console.log('Connection to Devnet Established Successfully!');

// reading balance of an account
const senderPvtKey = getKeypairFromEnvironment('SECRET_KEY');
const receiverPvtKey = getKeypairFromEnvironment('RECEIVER_SECRET_KEY');

const senderAddress = new PublicKey(senderPvtKey.publicKey.toBase58());
const receiverAddress = new PublicKey(receiverPvtKey.publicKey.toBase58());

// if (!senderAddress) {
//   console.log(`Please provide a public key to send to`);
//   process.exit(1);
// }

connection.getBalance(senderAddress).then((balance) => {
  console.log(`Before -- Balance of ${senderAddress.toBase58()}: ${balance}`);
});
connection.getBalance(receiverAddress).then((balance) => {
  console.log(`Before -- Balance of ${receiverAddress.toBase58()}: ${balance}`);
});

const transaction = new Transaction();
const amount = 0.001;
const LAMPORTS_TO_SEND = amount * LAMPORTS_PER_SOL;

const sendSolInstruction = SystemProgram.transfer({
  fromPubkey: senderAddress,
  toPubkey: receiverAddress,
  lamports: LAMPORTS_TO_SEND,
});

transaction.add(sendSolInstruction);
const signatureFn = async () => {
  const signature = await sendAndConfirmTransaction(connection, transaction, [
    senderPvtKey,
  ]);
  return signature;
};

signatureFn().then((signature) => {
  console.log(
    `💸 Finished! Sent ${LAMPORTS_TO_SEND} to the address ${receiverAddress}. `
  );
  console.log(`Transacti/on signature is ${signature}!`);
});
