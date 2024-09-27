import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import {Keypair, PublicKey} from "@solana/web3.js"
 

describe("counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Counter as Program<Counter>;

  // const counterAccount = new Keypair();

  const [counterPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter")],
    program.programId,
  )
;

  it("Is initialized!", async () => {
    // Add your test here.
    try{
      const txSig = await program.methods.initialize().rpc()
      
      const accountData = await program.account.counter.fetch(
        counterPDA
      );

      console.log(`Your transaction signature ${txSig}`);
      console.log(`Count: ${accountData.count}`);
    }catch(error){
      // If PDA Accounct created , log error
      console.log(error)
    }
     
  });
    
  it("Increment", async()=>{
    // Invoke the increment instruction
    const transactionSign = await program.methods.increment().rpc();

    const accountData = await program.account.counter.fetch(counterPDA)

    console.log(`Transaction Signature:${transactionSign}`)
    console.log(`Count:  ${accountData.count }`)
  })

});