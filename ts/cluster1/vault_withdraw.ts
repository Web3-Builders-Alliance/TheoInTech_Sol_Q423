import {
  Address,
  AnchorProvider,
  BN,
  Program,
  Wallet,
} from "@coral-xyz/anchor";
import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import wallet from "../wba-wallet.json";
import { IDL, WbaVault } from "./programs/wba_vault";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Commitment
const commitment: Commitment = "confirmed";

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment,
});

// Create our program
const program = new Program<WbaVault>(
  IDL,
  "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address,
  provider
);

// Create a random keypair
const vaultState = new PublicKey(
  "9VJLLb3vamekrRRm9ufW5KT41uDwGjWwQNa1AkzXktSM"
);
console.log(`Vault public key: ${vaultState.toBase58()}`);

// Create the PDA for our enrollment account
// Seeds are "auth", vaultState
const vaultAuth = [Buffer.from("auth"), vaultState.toBuffer()];
const [vaultAuthKey, _bump] = PublicKey.findProgramAddressSync(
  vaultAuth,
  program.programId
);

// Create the vault key
// Seeds are "vault", vaultAuth
const vault = [Buffer.from("vault"), vaultAuthKey.toBuffer()];
const [vaultKey, _bump2] = PublicKey.findProgramAddressSync(
  vault,
  program.programId
);

// Execute our enrollment transaction
(async () => {
  try {
    const signature = await program.methods
      .withdraw(new BN(LAMPORTS_PER_SOL))
      .accounts({
        owner: keypair.publicKey,
        vault: vaultKey,
        vaultState: vaultState,
        vaultAuth: vaultAuthKey,
        systemProgram: PublicKey.default,
      })
      .signers([keypair])
      .rpc();

    console.log(
      `Withdraw success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
    );

    // https://explorer.solana.com/tx/3wWiAHBXuCYTnJjDBbun9YAAj9yD5Mx6Yoc6g8TLtntaTRn7FjoxsxfuJWVRC1o8HxEdSyUUyXgudyEwN2i5tp8z?cluster=devnet
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
