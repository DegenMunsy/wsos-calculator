// Import the required modules from the anchor library
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Calculator } from "../target/types/calculator";
const { SystemProgram } = anchor.web3;
import { expect } from "chai";

// Describe a test suite for the calculator program
describe("calculator", () => {
  // Set the anchor provider to the local environment
  anchor.setProvider(anchor.AnchorProvider.env());

  // Create a reference to the calculator program
  const program = anchor.workspace.Calculator as Program<Calculator>;
  const programProvider = program.provider as anchor.AnchorProvider;

  // Generate a key pair for the calculator account
  const calculatorPair = anchor.web3.Keypair.generate();

  const text = "Summer School Of Solana";

  // Define a test case for creating a calculator instance
  it("Creating Calculator Instance", async () => {
    // Call the create method from the calculator program
    await program.methods
      .create(text)
      .accounts({
        calculator: calculatorPair.publicKey,
        user: programProvider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([calculatorPair])
      .rpc();

    // Fetch the calculator account and check if the greeting field is set correctly
    const account = await program.account.calculator.fetch(calculatorPair.publicKey);
    expect(account.greeting).to.eql(text);
  });

  // Define a test case for addition
  it("Addition", async () => {
    await program.methods
      .add(new anchor.BN(2), new anchor.BN(3))
      .accounts({
        calculator: calculatorPair.publicKey,
      })
      .rpc();

    // Fetch the calculator account and check if the result field is set correctly
    const account = await program.account.calculator.fetch(calculatorPair.publicKey);
    expect(account.result).to.eql(new anchor.BN(5));
  });


 // Define a test case for subtraction
 it("Subtraction", async () => {
   await program.methods
     .subtract(new anchor.BN(7), new anchor.BN(4))
     .accounts({
       calculator: calculatorPair.publicKey,
      })
      .rpc();

    // Fetch the calculator account and check if the result field is set correctly
    const account = await program.account.calculator.fetch(calculatorPair.publicKey);
    expect(account.result).to.eql(new anchor.BN(3));
  });

 // Define a test case for multiplication
 it("Multiplication", async () => {
   await program.methods
     .multiply(new anchor.BN(3), new anchor.BN(4))
     .accounts({
       calculator: calculatorPair.publicKey,
      })
      .rpc();

    // Fetch the calculator account and check if the result field is set correctly
    const account = await program.account.calculator.fetch(calculatorPair.publicKey);
    expect(account.result).to.eql(new anchor.BN(12));
  });

 // Define a test case for division
 it("Division", async () => {
   await program.methods
     .divide(new anchor.BN(12), new anchor.BN(4))
     .accounts({
       calculator: calculatorPair.publicKey,
      })
      .rpc();

    // Fetch the calculator account and check if the result field is set correctly
    const account = await program.account.calculator.fetch(calculatorPair.publicKey);
    expect(account.result).to.eql(new anchor.BN(3));
  });

});