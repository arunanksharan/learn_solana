import {
  Connection,
  PublicKey,
  clusterApiUrl,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import * as dotenv from "dotenv";
import { loaded_keypair } from "./generate_keypair";

dotenv.config({ path: "../.env" });

const connection = new Connection(clusterApiUrl("devnet"));
console.log("Connection to Devnet Established Successfully!");

// reading balance of an account
const address = new PublicKey(loaded_keypair.publicKey.toBase58());
connection.getBalance(address).then((balance) => {
  console.log(`Balance of ${address.toBase58()}: ${balance}`);
});

const balance = await connection.getBalance(address);
console.log(`The balance of the account at ${address} is ${balance} lamports`);
console.log(`✅ Finished!`);

const balanceInSol = balance / LAMPORTS_PER_SOL;

console.log(`The balance of the account at ${address} is ${balanceInSol} SOL`);
console.log(`✅ Finished!`);
