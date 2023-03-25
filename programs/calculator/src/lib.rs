// Import the anchor_lang prelude, which contains many commonly used items
use anchor_lang::prelude::*;

// Import the solana_program entrypoint module to handle program results
use anchor_lang::solana_program::entrypoint::ProgramResult;

// Declare the program ID, which is the unique identifier for your program on the Solana blockchain
declare_id!("fRprMGtWFJ9oGAjCedCc9xSfZoQfWZm43Z2pp29z5ML");

// Define the calculator module
#[program]
pub mod calculator {
    // Import the items from the parent scope
    use super::*;

    // Define the create function, which initializes the calculator account with a greeting
    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        // Get a mutable reference to the calculator account
        let calculator = &mut ctx.accounts.calculator;

        // Set the calculator account's greeting field to the input message
        calculator.greeting = init_message;

        // Return success
        Ok({})
    }

    // Define the add function, which calculates the sum of two numbers
    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> ProgramResult {
        // Get a mutable reference to the calculator account
        let calculator = &mut ctx.accounts.calculator;

        // Set the calculator account's result field to the sum of the input numbers
        calculator.result = num1 + num2;

        // Return success
        Ok(())
    }

    // Define the subtract function, which calculates the difference of two numbers
    pub fn subtract(ctx: Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult {
         let calculator = &mut ctx.accounts.calculator;
         calculator.result = num1 - num2;
         Ok(())
    }
    
        // Define the multiply function, which calculates the product of two numbers
    pub fn multiply(ctx: Context<Multiplication>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        Ok(())
    }
    
        // Define the divide function, which calculates the quotient of two numbers
    pub fn divide(ctx: Context<Division>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 / num2;
        Ok(())
    }
}

// Define the account structs used in the create function
#[derive(Accounts)]
pub struct Create<'info> {
    // Define the calculator account, which is initialized, paid for by the user, and has a size of 264 bytes
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,

    // Define the user account, which is mutable and a signer
    #[account(mut)]
    pub user: Signer<'info>,

    // Define the system_program account, which represents the Solana system program
    pub system_program: Program<'info, System>,
}

// Define the account structs used in the add function
#[derive(Accounts)]
pub struct Addition<'info> {
    // Define the calculator account, which is mutable
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

// Define the account structs used in the subtract function
#[derive(Accounts)]
pub struct Subtraction<'info> {
    // Define the calculator account, which is mutable
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

// Define the account structs used in the multiply function
#[derive(Accounts)]
pub struct Multiplication<'info> {
    // Define the calculator account, which is mutable
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

// Define the account structs used in the divide function
#[derive(Accounts)]
pub struct Division<'info> {
    // Define the calculator account, which is mutable
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

// Define the Calculator account data structure
#[account]
pub struct Calculator {
    // Declare a greeting field of type String
    greeting: String,

    // Declare a result field of type i64 (64-bit signed integer)
    result: i64
}



