import * as dotenv from "dotenv";
import { Keypair } from "@solana/web3.js";
import { getKeypairFromEnvironment } from "@solana-developers/node-helpers";

dotenv.config({ path: "../.env" });

export const loaded_keypair = getKeypairFromEnvironment("SECRET_KEY");
console.log("Loaded Keypair Successfully from env file!");
console.log(`PublicKey: ${loaded_keypair.publicKey.toBase58()}`);
console.log(`Private Key: ${loaded_keypair.secretKey.toString()}`);

// const new_keypair = Keypair.generate();
// console.log("Generated Keypair Successfully!")
// console.log(`PublicKey: ${new_keypair.publicKey.toBase58()}`);
// console.log(`Private Key: ${new_keypair.secretKey.toString()}`);
