const assert = require("assert");
const anchor = require("@coral-xyz/anchor");
const {
  VersionedTransaction, TransactionMessage, Transaction,
  AddressLookupTableProgram, AccountMeta
} = require("@solana/web3.js");

const { SystemProgram } = anchor.web3;

describe("basic-3", () => {
  const provider = anchor.AnchorProvider.local();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  // it("address lookup table", async () => {
  //   const invocation = anchor.workspace.Invocation;
  //   const newAccount0 = anchor.web3.Keypair.generate();
  //   const newAccount1 = anchor.web3.Keypair.generate();
  //   console.log("balance:", await provider.connection.getBalance(provider.publicKey));

  //   const slot = await provider.connection.getSlot('finalized')

  //   const [loookupTableInstruction, lookupTableAddress] = AddressLookupTableProgram.createLookupTable({
  //     authority: provider.publicKey,
  //     payer: provider.publicKey,
  //     recentSlot: slot,
  //   });

  //   const extendInstruction = AddressLookupTableProgram.extendLookupTable({
  //     payer: provider.publicKey,
  //     authority: provider.publicKey,
  //     lookupTable: lookupTableAddress,
  //     addresses: [newAccount0.publicKey, newAccount1.publicKey],
  //   });

  //   let createLookupTableTx = new Transaction();
  //   createLookupTableTx.add(loookupTableInstruction);
  //   createLookupTableTx.add(extendInstruction);

  //   console.log("lookup table address:", lookupTableAddress.toBase58());
  //   await provider.sendAndConfirm(createLookupTableTx, []);

  //   const lookupTableAccount = await provider.connection
  //     .getAddressLookupTable(lookupTableAddress)
  //     .then((res) => res.value);

  //   for (let i = 0; i < lookupTableAccount.state.addresses.length; i++) {
  //     const address = lookupTableAccount.state.addresses[i];
  //     console.log("address index:", i, address.toBase58());
  //   }

  //   const instruction = await invocation.methods.addressLookup().accounts({
  //     account0: newAccount0.publicKey,
  //     account1: newAccount1.publicKey
  //   }).instruction();

  //   let txWithLookupTable = new VersionedTransaction(
  //     new TransactionMessage({
  //       instructions: [instruction],
  //       payerKey: provider.publicKey,
  //       recentBlockhash: (await provider.connection.getLatestBlockhash())
  //         .blockhash,
  //     }).compileToV0Message([lookupTableAccount])
  //   );

  //   let txWithoutLookupTable = new VersionedTransaction(
  //     new TransactionMessage({
  //       instructions: [instruction],
  //       payerKey: provider.publicKey,
  //       recentBlockhash: (await provider.connection.getLatestBlockhash())
  //         .blockhash,
  //     }).compileToV0Message()
  //   );

  //   // must delay/sleep
  //   // see: https://solana.stackexchange.com/questions/2896/what-does-transaction-address-table-lookup-uses-an-invalid-index-mean
  //   await delay(2000);


  //   console.log('Transaction size without address lookup table: ', txWithoutLookupTable.serialize().length, 'bytes');
  //   console.log('Transaction size with address lookup table:    ', txWithLookupTable.serialize().length, 'bytes');

  //   await provider.sendAndConfirm(txWithLookupTable, [], {
  //     skipPreflight: false,
  //     commitment: "confirmed",
  //   });
  // })

  it("Performs CPI from puppet master to puppet", async () => {
    const invocation = anchor.workspace.Invocation;
    const implementation = anchor.workspace.Implementation0;
    // const implementation = anchor.workspace.Implementation1;

    // Initialize a new puppet account.
    const newAccount = anchor.web3.Keypair.generate();
    const newAccount2 = anchor.web3.Keypair.generate();
    const newAccount3 = anchor.web3.Keypair.generate();

    let endpoint = anchor.web3.PublicKey.createProgramAddressSync(
      [Buffer.from('seed', 'utf8'), Buffer.from([253])],
      implementation.programId
    );
    console.log("endpoint", endpoint);

    await implementation.methods
      .initialize()
      .accounts({
        account: newAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        // endpoint,
      })
      .signers([newAccount])
      .rpc();

    await implementation.methods
      .initialize()
      .accounts({
        account: newAccount3.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        // endpoint,
      })
      .signers([newAccount3])
      .rpc();

    // Invoke the puppet master to perform a CPI to the puppet.
    // const a = await invocation.methods
    //   .setData()
    //   .accounts({
    //     signer: newAccount.publicKey,
    //     payer: newAccount2.publicKey,
    //     endpoint: anchor.web3.PublicKey.createProgramAddressSync(
    //       [Buffer.from('seed', 'utf8'), Buffer.from([253])],
    //       implementation.programId
    //     ),
    //   }).accounts;
    // console.log(a);

    await invocation.methods
      .invoke()
      .accounts({
        signer: newAccount.publicKey,
        payer: newAccount2.publicKey,
        programId: implementation.programId,
      })
      .remainingAccounts([
        { pubkey: newAccount3.publicKey, isSigner: false, isWritable: true }, // account
        { pubkey: newAccount2.publicKey, isSigner: true, isWritable: true }, // payer
        { pubkey: endpoint, isSigner: false, isWritable: false }, // endpoint
        //
        { pubkey: anchor.web3.Keypair.generate().publicKey, isSigner: false, isWritable: true },
        { pubkey: anchor.web3.Keypair.generate().publicKey, isSigner: false, isWritable: true },
        { pubkey: anchor.web3.Keypair.generate().publicKey, isSigner: false, isWritable: true },
      ])
      .signers([newAccount, newAccount2])
      .rpc();

    // Check the state updated.
    puppetAccount = await implementation.account.data.fetch(newAccount.publicKey);
    assert.ok(puppetAccount.data.eq(new anchor.BN(111)));
  });
});

function delay(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}