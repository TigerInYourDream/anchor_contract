import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorContract } from "../target/types/anchor_contract";
import {
    TOKEN_PROGRAM_ID,
    MINT_SIZE,
    createAssociatedTokenAccount,
    getAssociatedTokenAddress,
    createInitializeMintInstruction,
    createMintToInstruction,
    createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";

describe("anchor_contract", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.AnchorContract as Program<AnchorContract>;
    const mintKey = anchor.web3.Keypair.generate()
    let associatedTokenAccount;

    // create fake wallet tokens and ATA for our wallet

    it("Mint token with supply 10", async () => {
        // Add your test here.
        const key = anchor.AnchorProvider.env().wallet.publicKey;
        console.log("Your public key", key.toBase58());
        const lamports: number = await program.provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE);
        // get ATA Associated Token Account
        associatedTokenAccount = await getAssociatedTokenAddress(
            mintKey.publicKey,
            key,
        );

        // Fires a list of instractions
        const mint_tx = new anchor.web3.Transaction().add(
            // use anchor to create an account from the key we generated
            anchor.web3.SystemProgram.createAccount({
                fromPubkey: key,
                newAccountPubkey: mintKey.publicKey,
                space: MINT_SIZE,
                programId: TOKEN_PROGRAM_ID,
                lamports,
            }),
            createInitializeMintInstruction(
                mintKey.publicKey,
                // decimals
                0,
                key,
                key
            ),
            createAssociatedTokenAccountInstruction(
                key,
                associatedTokenAccount,
                key,
                mintKey.publicKey,
            ),
        );

        const res = await anchor.AnchorProvider.env().sendAndConfirm(mint_tx, [mintKey]);
        console.log(
            await program.provider.connection.getParsedAccountInfo(mintKey.publicKey)
        );

        console.log("Account :", res);
        console.log("ATA :", associatedTokenAccount.toBase58());
        console.log("MintKey :", mintKey.publicKey.toBase58());
        console.log("UserKey :", key.toBase58());

        // execute the mint to the ATA
        const tx = await program.methods.mintToken().accounts(
            {
                mint: mintKey.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                tokenAccout: associatedTokenAccount,
                payer: key,
            }
        ).rpc();
        console.log("You transaction ", tx);
    });
})
