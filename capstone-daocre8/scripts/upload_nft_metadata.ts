import {
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createBundlrUploader } from "@metaplex-foundation/umi-uploader-bundlr";
import wallet from "../wba-wallet.json";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");
const bundlrUploader = createBundlrUploader(umi);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));

(async () => {
  try {
    // Follow this JSON structure
    // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure
    const imageCreator =
      "https://arweave.net/lHs2i1W4FOm_ug_8odTm-rJy7kcWiskKkVScBWP5pEE";

    const metadataCreator = {
      name: "DAOCre-8 Creator",
      symbol: "DCC",
      description: "DAOCre-8 Creator",
      image: imageCreator,
      properties: {
        files: [
          {
            type: "image/png",
            uri: imageCreator,
          },
        ],
      },
      creators: [],
    };

    const uriCreator = await bundlrUploader.uploadJson(metadataCreator);
    console.log("Your Creator Metadata: ", uriCreator);
    // Your Creator Metadata:  https://arweave.net/V-5alv-OMXg05vz_XMFp2WwE3c7-H4SMIeDuFYOFza0
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();
