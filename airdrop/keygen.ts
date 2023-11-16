import { Keypair } from "@solana/web3.js";

// Generate a new keypair
let kp = Keypair.generate();
console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`);

// 7qsggUFSA6wVFnh6XgC5MiBJikaP81fsKKbptW1o7kaW

console.log(
  `To save your wallet, copy and paste your private key into a JSON file: ${kp.secretKey}`
);
