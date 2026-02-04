import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { assert } from "chai";

describe("counter", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;

  // Generate a new keypair for our counter account
  const counterKeypair = anchor.web3.Keypair.generate();

  it("Initializes the counter to 0", async () => {
    // Call the initialize instruction
    const tx = await program.methods
      .initialize()
      .accounts({
        counter: counterKeypair.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([counterKeypair])
      .rpc();

    console.log("Initialize tx:", tx);

    // Fetch the counter account and verify it's 0
    const counterAccount = await program.account.counter.fetch(counterKeypair.publicKey);
    assert.equal(counterAccount.count.toNumber(), 0);
    console.log("Counter initialized to:", counterAccount.count.toNumber());
  });

  it("Increments the counter", async () => {
    // Call the increment instruction
    const tx = await program.methods
      .increment()
      .accounts({
        counter: counterKeypair.publicKey,
      })
      .rpc();

    console.log("Increment tx:", tx);

    // Fetch the counter account and verify it's now 1
    const counterAccount = await program.account.counter.fetch(counterKeypair.publicKey);
    assert.equal(counterAccount.count.toNumber(), 1);
    console.log("Counter incremented to:", counterAccount.count.toNumber());
  });

  it("Increments the counter again", async () => {
    // Call increment again
    const tx = await program.methods
      .increment()
      .accounts({
        counter: counterKeypair.publicKey,
      })
      .rpc();

    console.log("Increment tx:", tx);

    // Verify it's now 2
    const counterAccount = await program.account.counter.fetch(counterKeypair.publicKey);
    assert.equal(counterAccount.count.toNumber(), 2);
    console.log("Counter incremented to:", counterAccount.count.toNumber());
  });
});
