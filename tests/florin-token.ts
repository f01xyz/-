import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FlorinToken } from "../target/types/florin_token";
import {
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddress,
  getAccount,
} from "@solana/spl-token";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { assert } from "chai";

describe("florin-token", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.FlorinToken as Program<FlorinToken>;
  
  // Generate a new keypair for the mint
  const mintKeypair = anchor.web3.Keypair.generate();
  
  // Use the provider's wallet as the mint authority and payer
  const mintAuthority = provider.wallet;
  const payer = provider.wallet;
  
  // Create a keypair for a contributor
  const contributor = anchor.web3.Keypair.generate();
  
  // Token account for the contributor
  let contributorTokenAccount: PublicKey;
  
  // Mint decimals
  const decimals = 6;
  
  // Test amount to mint and burn
  const mintAmount = 1000000000; // 1000 tokens with 6 decimals
  const burnAmount = 500000000;  // 500 tokens with 6 decimals
  
  before(async () => {
    // Airdrop SOL to the contributor for transaction fees
    const airdropSig = await provider.connection.requestAirdrop(
      contributor.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);
  });

  it("Initializes the Florin token mint", async () => {
    // Initialize the mint
    const tx = await program.methods
      .initializeMint(decimals, mintAuthority.publicKey)
      .accounts({
        mint: mintKeypair.publicKey,
        mintAuthority: mintAuthority.publicKey,
        payer: payer.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([mintKeypair, mintAuthority.payer])
      .rpc();
    
    console.log("Initialize mint transaction signature", tx);
    
    // Create the contributor's token account
    contributorTokenAccount = await getAssociatedTokenAddress(
      mintKeypair.publicKey,
      contributor.publicKey
    );
    
    // Create the associated token account for the contributor
    const createTokenAccountTx = new anchor.web3.Transaction().add(
      createAssociatedTokenAccountInstruction(
        payer.publicKey,
        contributorTokenAccount,
        contributor.publicKey,
        mintKeypair.publicKey
      )
    );
    
    await provider.sendAndConfirm(createTokenAccountTx);
    console.log("Created contributor token account:", contributorTokenAccount.toString());
  });

  it("Mints Florin tokens to a contributor", async () => {
    // Create metadata for the contribution
    const metadata = {
      voucherId: "voucher-123",
      contributorRef: "contributor-xyz",
      campaign: "initial-funding",
      notes: "First contribution to the network",
    };
    
    // Mint tokens to the contributor
    const tx = await program.methods
      .mintFlorin(
        new anchor.BN(mintAmount),
        {
          voucherId: metadata.voucherId,
          contributorRef: metadata.contributorRef,
          campaign: metadata.campaign,
          notes: metadata.notes,
        }
      )
      .accounts({
        mint: mintKeypair.publicKey,
        mintAuthority: mintAuthority.publicKey,
        recipient: contributorTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([mintAuthority.payer])
      .rpc();
    
    console.log("Mint transaction signature", tx);
    
    // Verify the tokens were minted
    const tokenAccount = await getAccount(
      provider.connection,
      contributorTokenAccount
    );
    
    assert.equal(
      tokenAccount.amount.toString(),
      mintAmount.toString(),
      "Token balance should match the minted amount"
    );
  });

  it("Burns Florin tokens", async () => {
    // Burn tokens from the contributor's account
    const tx = await program.methods
      .burnFlorin(new anchor.BN(burnAmount))
      .accounts({
        mint: mintKeypair.publicKey,
        from: contributorTokenAccount,
        owner: contributor.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([contributor])
      .rpc();
    
    console.log("Burn transaction signature", tx);
    
    // Verify the tokens were burned
    const tokenAccount = await getAccount(
      provider.connection,
      contributorTokenAccount
    );
    
    const expectedRemainingAmount = mintAmount - burnAmount;
    assert.equal(
      tokenAccount.amount.toString(),
      expectedRemainingAmount.toString(),
      "Token balance should be reduced by the burned amount"
    );
  });
});
