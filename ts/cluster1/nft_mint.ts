import {
  createNft,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  generateSigner,
  percentAmount,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";

import { base58 } from "@metaplex-foundation/umi/serializers";
import wallet from "../wba-wallet.json";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());

const mint = generateSigner(umi);

(async () => {
  let tx = await createNft(umi, {
    mint,
    name: "Generug x TheoInTech",
    uri: "https://dcgnkse2gqrpxas6v3ijyjecukw3repjgribvr5npv5pmgr37cia.arweave.net/GIzVSJo0IvuCXq7QnCSCoq24kek0UBrHrX169ho7-JA",
    sellerFeeBasisPoints: percentAmount(69),
    symbol: "RUG",
  });
  let result = await tx.sendAndConfirm(umi);
  const signature = base58.deserialize(result.signature);

  console.log(
    `Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
  );

  // https://explorer.solana.com/tx/Va7sEe88gHnAaQh4LwfCCh7Nx1ELpJBa8NVpqJ544R4WhdWf22dGchMQYsByNdyyfPLLU57B9MPFR5V6eD7J5ha?cluster=devnet
  // Mint Address: DQLYQt6c9gjhvNWiZNFMT8sGMwnDBVzbjXA6CtMyX8Gu

  console.log("Mint Address: ", mint.publicKey);
})();
