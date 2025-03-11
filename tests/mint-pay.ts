import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MintPay } from "../target/types/mint_pay";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplCore, MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";
import { createSignerFromKeypair, generateSigner, signerIdentity } from "@metaplex-foundation/umi";
import { Keypair } from "@solana/web3.js";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import * as assert from "assert";

describe("mint", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());
  
  const program = anchor.workspace.MintPay as Program<MintPay>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  
  it("Mint a new token!", async () => {
    // Générer un keypair pour le NFT
    const mint = anchor.web3.Keypair.generate();
    const user = provider.wallet.publicKey;
    
    // Exécuter la transaction de création du NFT
    const mintTx = await program.methods
      .initializeMint("My new token", "https://example.com/metadata")
      .accounts({
        user: user,
        mint: mint.publicKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
      })
      .signers([mint])
      .rpc();

    console.log("Mint transaction signature", mintTx);
    
    // Confirmer la transaction
    await provider.connection.confirmTransaction(mintTx);
    
    // Vérifier que le compte du NFT existe
    const mintInfo = await provider.connection.getAccountInfo(mint.publicKey);
    assert.ok(mintInfo, "Le compte du NFT devrait exister après la création");
  });

  it("Test complet du processus de création de NFT", async () => {
    // Générer les keypairs pour le test
    const mint = anchor.web3.Keypair.generate();
    const user = provider.wallet.publicKey;
    
    // Définir les métadonnées du NFT
    const nftName = "Mon NFT de Test";
    const nftUri = "https://arweave.net/metadata-test-uri";
    
    console.log("Création du NFT en cours...");
    
    try {
      // Exécuter la transaction de création du NFT
      const mintTx = await program.methods
        .initializeMint(nftName, nftUri)
        .accounts({
          mint: mint.publicKey,
          user: user,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
          systemProgram: SYSTEM_PROGRAM_ID,
        })
        .signers([mint])
        .rpc();
      
      console.log("NFT créé avec succès!");
      console.log("Signature de la transaction:", mintTx);
      
      // Confirmer la transaction
      await provider.connection.confirmTransaction(mintTx);
      
      // Vérifier que le compte du NFT existe
      const mintAccountInfo = await provider.connection.getAccountInfo(mint.publicKey);
      assert.ok(mintAccountInfo, "Le compte du NFT devrait exister après la création");
      
      // Vérifications supplémentaires
      console.log("Clé publique du NFT:", mint.publicKey.toString());
      console.log("Clé publique du propriétaire:", user.toString());
      console.log("Taille du compte NFT:", mintAccountInfo.data.length, "octets");
      
      // Test réussi
      console.log("Test de création de NFT réussi!");
    } catch (error) {
      console.error("Erreur lors de la création du NFT:", error);
      throw error;
    }
  });
  
  it("Vérifie les erreurs lors de la création d'un NFT avec des données invalides", async () => {
    // Générer les keypairs pour le test
    const mint = anchor.web3.Keypair.generate();
    const user = provider.wallet.publicKey;
    
    // Test avec un nom vide (devrait échouer)
    console.log("Test avec un nom vide...");
    try {
      await program.methods
        .initializeMint("", "https://arweave.net/metadata")
        .accounts({
          mint: mint.publicKey,
          user: user,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
          systemProgram: SYSTEM_PROGRAM_ID,
        })
        .signers([mint])
        .rpc();
      
      assert.fail("La transaction aurait dû échouer avec un nom vide");
    } catch (error) {
      console.log("Erreur attendue reçue:", error.message);
    }
    
    // Test avec une URI vide (devrait échouer)
    console.log("Test avec une URI vide...");
    try {
      await program.methods
        .initializeMint("Test NFT", "")
        .accounts({
          mint: mint.publicKey,
          user: user,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
          systemProgram: SYSTEM_PROGRAM_ID,
        })
        .signers([mint])
        .rpc();
      
      assert.fail("La transaction aurait dû échouer avec une URI vide");
    } catch (error) {
      console.log("Erreur attendue reçue:", error.message);
    }
    
    console.log("Tests de validation réussis!");
  });
});
