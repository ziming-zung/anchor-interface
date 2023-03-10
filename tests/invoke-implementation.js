const assert = require("assert");
const anchor = require("@coral-xyz/anchor");
const { SystemProgram } = anchor.web3;

describe("basic-3", () => {
  const provider = anchor.AnchorProvider.local();
  
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  it("Performs CPI from puppet master to puppet", async () => {
    const invocation = anchor.workspace.Invocation;
    const implementation = anchor.workspace.Implementation1;
    // const implementation = anchor.workspace.Implementation1;
    
    // Initialize a new puppet account.
    const newAccount = anchor.web3.Keypair.generate();
    const tx = await implementation.methods
      .initialize()
      .accounts({
        account: newAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([newAccount])
      .rpc();

    // Invoke the puppet master to perform a CPI to the puppet.
    await invocation.methods
      .invoke()
      .accounts({
        signer: newAccount.publicKey,        
        programId: implementation.programId,
      }).signers([newAccount])
      .rpc();

    // Check the state updated.
    puppetAccount = await implementation.account.data.fetch(newAccount.publicKey);
    assert.ok(puppetAccount.data.eq(new anchor.BN(111)));
  });
});

function delay(ms) {
  return new Promise( resolve => setTimeout(resolve, ms) );
}