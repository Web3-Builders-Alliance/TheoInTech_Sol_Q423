import { createTree, mplBubblegum } from "@metaplex-foundation/mpl-bubblegum";
import { mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  generateSigner,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { base58 } from "@metaplex-foundation/umi/serializers";

import wallet from "../wba-wallet.json";

(async () => {
  // const RPC_ENDPOINT = "http://127.0.0.1:8899";
  const RPC_ENDPOINT = "https://api.devnet.solana.com";
  const umi = createUmi(RPC_ENDPOINT).use(mplBubblegum());

  let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
  const myKeypairSigner = createSignerFromKeypair(umi, keypair);
  umi.use(signerIdentity(myKeypairSigner));
  umi.use(mplTokenMetadata());

  console.log("Creating create tree...");
  const merkleTree = generateSigner(umi);
  const builder = await createTree(umi, {
    merkleTree,
    maxDepth: 17,
    maxBufferSize: 64,
  });
  let createResult = await builder.sendAndConfirm(umi);
  const signature = base58.deserialize(createResult.signature);
  console.log(`Succesfully Created Tree! Signature: ${signature}`);
  console.log("Merkle Tree: ", merkleTree.publicKey);

  // https://explorer.solana.com/tx/uCQYGMoFzF1S3yCZsQTQVTR3eDjkVTp9Cc4RmKvJrmD6pZ1zxPsDGhwAPdR7VaHFPXzg5ykr2Xx7GS8zmzwRGp2?cluster=devnet
  // Merkle Tree Address: CNr5kujH2UPbLVNQ6wZ3NkfEwiQJuR9BD2GYTGWAj1i4
})();
