import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolOperation } from "../target/types/sol_operation";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import fs from "fs";
describe("sol-operation", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const owner = provider.publicKey;
  const program = anchor.workspace.SolOperation as Program<SolOperation>;
  const sender = owner;
  const reciver = new anchor.web3.PublicKey("2hedrK5zo6LqJU9SMZNJZwz2FmD4rPDK4kAaPds4EnHh");
  const DATA = utf8.encode("ABC");
  const txis:anchor.web3.TransactionInstruction[]=[];
  const [userpda,bump] = anchor.web3.PublicKey.findProgramAddressSync([DATA,owner.toBuffer()],program.programId);
  const [reciverpda,_] = anchor.web3.PublicKey.findProgramAddressSync([DATA,reciver.toBuffer()],program.programId);
  console.log("ReciverPda",reciverpda);
  
  const seedString = fs.readFileSync("./_user/a.json",{encoding:"utf8"});
  const seedJson = JSON.parse(seedString);
  const seedBuffer = Uint8Array.from(seedJson);
  const myAccount = anchor.web3.Keypair.fromSecretKey(seedBuffer);
  
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize()
    .accounts({
      systemProgram: anchor.web3.SystemProgram.programId,
      owner:myAccount,
      user:reciver,
      userInfo:reciverpda,
    }).instruction();
    txis.push(tx);
    const transaction = new anchor.web3.Transaction().add(tx);
    const sign = await provider.sendAndConfirm(transaction);
    console.log("Your transaction signature", sign);
  });

  // it("Sol Transfer", async() => {
  //   const txi = await program.methods.solTransfer(new anchor.BN(1))
  //   .accounts({
  //     owner:owner,
  //     reciver:reciver,
  //     sender:sender,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).instruction();

  //   txis.push(txi);

  //   const transaction = new anchor.web3.Transaction().add(...txis);
  //   const sign = await provider.sendAndConfirm(transaction);

  //   console.log("Signature:",sign);
  // });

  // it("Sol Transfer From User To PDA", async() => {
  //   const txi = await program.methods.transferSolPda(new anchor.BN(2))
  //   .accounts({
  //     owner:owner,
  //     userPda:userpda,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).instruction();

  //   txis.push(txi);

  //   const transaction = new anchor.web3.Transaction().add(...txis);
  //   const sign = await provider.sendAndConfirm(transaction);

  //   console.log("Signature:",sign);
  // });
});
