import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { AnchorVault } from "../target/types/anchor_vault";

describe("anchor-vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const signer = anchor.web3.Keypair.generate();

  const program = anchor.workspace.AnchorVault as Program<AnchorVault>;

  // Create a devnet connection
  const connection = anchor.getProvider().connection;

  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), signer.publicKey.toBuffer()],
    program.programId
  )[0];

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({ signature, ...block });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  it("Airdrop 10 SOL to signer", async () => {
    await connection
      .requestAirdrop(signer.publicKey, LAMPORTS_PER_SOL * 10)
      .then(confirm)
      .then(log);
  });

  it("Deposit 2 SOL to the vault", async () => {
    const tx = await program.methods
      .deposit(new BN(LAMPORTS_PER_SOL * 2))
      .accounts({
        signer: signer.publicKey,
        vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc()
      .then(confirm)
      .then(log);
    console.log("Your transaction signature", tx);
  });

  it("Close the vault", async () => {
    const tx = await program.methods
      .close()
      .accounts({
        signer: signer.publicKey,
        vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc()
      .then(confirm)
      .then(log);
    console.log("Your transaction signature", tx);
  });
});
