import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createBundlrUploader } from "@metaplex-foundation/umi-uploader-bundlr";
import { readFile } from "fs/promises";
import wallet from "../wba-wallet.json";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");
const bundlrUploader = createBundlrUploader(umi);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));

(async () => {
  try {
    let fileCreator = await readFile("assets/nft_creator.png");
    const imageCreator = createGenericFile(fileCreator, "DAOCre-8 Creator", {
      contentType: "image/png",
    });
    const [uriCreator] = await bundlrUploader.upload([imageCreator]);
    console.log("Your Creator image URI: ", uriCreator);
    // Your Creator image URI:  https://arweave.net/lHs2i1W4FOm_ug_8odTm-rJy7kcWiskKkVScBWP5pEE
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();
