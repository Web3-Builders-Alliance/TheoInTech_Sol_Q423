import { createMetadataAccountV3 } from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  publicKey,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { PublicKey } from "@solana/web3.js";
import wallet from "../wba-wallet.json";

//Create a Solana devnet UMI connection
const umi = createUmi("https://api.devnet.solana.com");

// We're going to import our keypair from the wallet file using umi EdDSA interface
let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));

//Create a KeypairSigner and set Identity and Payer of the signer
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));

// Define our Mint address ("Standard Way")
// To define it using UMI pubKey method, it can be created as:
// const mint = publicKey("7VnSNSWRpu4VuAaxMKXrAJKKLJgdNeZGWXoWifSmSGFj");
// and them mint can be used directly in the createMetadataAccountV3 method without needing the toString() method
const mint = new PublicKey("7VnSNSWRpu4VuAaxMKXrAJKKLJgdNeZGWXoWifSmSGFj");

// Add the Token Metadata Program
const token_metadata_program_id = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

// Create PDA for token metadata
// Again, if the "Standard Method is not desired, the PDA can be created by serializing the seeds as Uint8array as:"
// const seeds =
//   [string({ size: 'variable' }).serialize('metadata'),
//   publicKeySerializer().serialize(tokenMetadataProgramId),
//   publicKeySerializer().serialize(mint),
// ];
// const metadata_pda = umi.eddsa.findPda(tokenMetadataProgramId, seeds);
const metadata_seeds = [
  Buffer.from("metadata"),
  token_metadata_program_id.toBuffer(),
  mint.toBuffer(),
];
const [metadata_pda, _bump] = PublicKey.findProgramAddressSync(
  metadata_seeds,
  token_metadata_program_id
);

(async () => {
  try {
    let myTransaction = createMetadataAccountV3(umi, {
      //accounts
      metadata: publicKey(metadata_pda.toString()),
      mint: publicKey(mint.toString()),
      mintAuthority: myKeypairSigner,
      payer: myKeypairSigner,
      updateAuthority: keypair.publicKey,
      data: {
        name: "TheoInTech",
        symbol: "TIT",
        uri: "https://theoin.tech",
        sellerFeeBasisPoints: 0,
        creators: null,
        collection: null,
        uses: null,
      },
      isMutable: true,
      collectionDetails: null,
    });

    let result = await myTransaction.sendAndConfirm(umi);

    console.log(result.signature);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
