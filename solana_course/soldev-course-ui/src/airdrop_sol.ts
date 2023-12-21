import {
  Keypair,
  Connection,
  clusterApiUrl,
  LAMPORTS_PER_SOL,
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
connection
  .requestAirdrop(payer.publicKey, LAMPORTS_PER_SOL * 1)
  .then((signature) => {
    console.log(`Airdrop Signature: ${signature}`);
  });

// Sample Airdrop
// 5dZgwekPS4io6wZeKHpWzbnzgkb9wjxwhWB4f6WFL7szm43s6c6NgXWgrVhjDZMWgw2Br8ebEcRCNk9ntyDdbCNg
