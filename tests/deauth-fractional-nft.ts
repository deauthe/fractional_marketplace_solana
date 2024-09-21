import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FractionalNft } from "../target/types/fractional_nft";

describe("deauth-fractional-nft", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .DeauthFractionalNft as Program<FractionalNft>;
  const user = anchor.web3.Keypair.generate();
  const creator = user.publicKey;
  const nft = anchor.web3.Keypair.generate();
  const business = anchor.web3.Keypair.generate();

  console.log(
    "user:",
    user.publicKey,
    "creator:",
    creator,
    "nft:",
    nft,
    "business:",
    business.publicKey
  );
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initializeNft("hello")
      .accounts({
        creator,
        nft: nft.publicKey,
        business: business.publicKey,
      })
      .signers([user, business, nft])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
