// ============================================================================
// SOLANA COUNTER PROGRAM - A beginner's guide with exhaustive comments
// ============================================================================

// "use" is like "import" in JavaScript/Python. It brings code from other
// places into this file so we can use it.
//
// "anchor_lang" is the Anchor framework library.
// "::prelude::*" means "from the prelude module, import everything".
// The "*" is a wildcard meaning "all public items".
// "prelude" is a common Rust convention - it's a module containing the most
// commonly used items so you can import them all at once.
use anchor_lang::prelude::*;

// This macro sets the program's unique address on the Solana blockchain.
//
// "declare_id!" is a macro (macros end with "!" in Rust). Macros are like
// functions that write code for you at compile time.
//
// The string inside is a base58-encoded public key (32 bytes). This is
// generated when you run "anchor build" for the first time.
// Every program deployed to Solana has a unique address, just like every
// website has a unique URL.
declare_id!("2LUoJnKc5maGZYcyqMcGk2WwdKwTygxwRP7uqBUu6h6y");

// "#[program]" is an "attribute macro". Attributes start with "#[" and end
// with "]". They modify the thing that comes after them.
//
// This attribute tells Anchor: "the module below contains the program's
// instruction handlers" (the functions users can call).
#[program]

// "pub" means "public" - this can be accessed from outside this file.
// "mod" declares a "module" - a way to organize code in Rust (like a folder).
// "counter" is the name we're giving this module.
// "{" starts the module's body (everything inside the curly braces belongs
// to this module).
pub mod counter {

    // "use" imports items. "super" refers to the parent scope (the code
    // outside this module). "::*" means "import everything from there".
    // This lets us use the imports from line 14 inside this module.
    use super::*;

    // ========================================================================
    // INSTRUCTION #1: initialize
    // Creates a new counter account and sets its value to 0
    // ========================================================================

    // "pub" = public (can be called from outside).
    // "fn" = "function" (declares a function).
    // "initialize" = the name of this function/instruction.
    //
    // "(ctx: Context<Initialize>)" = this function takes one parameter:
    //   - "ctx" is the parameter name (short for "context")
    //   - ":" separates the name from its type
    //   - "Context<Initialize>" is the type. "Context" is a generic type
    //     from Anchor, and "<Initialize>" specifies which accounts struct
    //     to use (defined below on line 97).
    //
    // "->" indicates what the function returns.
    // "Result<()>" is the return type:
    //   - "Result" is an enum that's either Ok (success) or Err (failure)
    //   - "<()>" means on success, we return "()" which is the "unit type"
    //     (Rust's version of void/nothing/null - no meaningful return value)
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        // "let" declares a new variable.
        // "counter" is the variable name.
        // "=" assigns a value to it.
        //
        // "&mut" means "mutable reference". Let me break this down:
        //   - "&" means "reference" (like a pointer - we're borrowing the
        //     data, not copying it)
        //   - "mut" means "mutable" (we're allowed to modify it)
        //   - Without "mut", Rust variables are immutable (read-only) by
        //     default!
        //
        // "ctx" is our context parameter from above.
        // ".accounts" accesses the "accounts" field of the context (this
        //   contains all the accounts passed to this instruction).
        // ".counter" accesses the specific account named "counter" (this
        //   name matches the field in the Initialize struct below).
        let counter = &mut ctx.accounts.counter;

        // "counter.count" accesses the "count" field of the counter account.
        // "= 0" sets it to zero.
        // ";" ends the statement (most lines in Rust end with semicolons).
        counter.count = 0;

        // "msg!" is a macro that logs a message to Solana's transaction logs.
        // Similar to console.log() in JavaScript.
        //
        // The string uses "{}" as a placeholder (like %s in other languages).
        // "counter.count" is the value that replaces "{}".
        msg!("Counter initialized! Current count: {}", counter.count);

        // "Ok(())" returns a successful result.
        // "Ok" is one variant of the Result enum (the success case).
        // "()" inside is the value we're returning (nothing/unit type).
        //
        // Note: no semicolon here! In Rust, the last expression in a function
        // without a semicolon is the return value. Adding ";" would make this
        // a statement instead of a return expression.
        Ok(())

    // "}" closes the function body.
    }

    // ========================================================================
    // INSTRUCTION #2: increment
    // Adds 1 to an existing counter
    // ========================================================================

    // Same structure as initialize, but uses "Context<Increment>" because
    // this instruction needs different accounts (see Increment struct below).
    pub fn increment(ctx: Context<Increment>) -> Result<()> {

        // Get a mutable reference to the counter account.
        // Same pattern as in initialize.
        let counter = &mut ctx.accounts.counter;

        // "+= 1" adds 1 to the current value (same as "counter.count =
        // counter.count + 1").
        counter.count += 1;

        // Log the new count.
        msg!("Counter incremented! Current count: {}", counter.count);

        // Return success.
        Ok(())
    }

// "}" closes the "counter" module.
}

// ============================================================================
// ACCOUNT STRUCT: Counter
// Defines what data is stored in a counter account
// ============================================================================

// "#[account]" is an attribute macro from Anchor that:
//   1. Implements serialization (converting struct to bytes for storage)
//   2. Implements deserialization (converting bytes back to struct)
//   3. Adds an 8-byte "discriminator" (a unique ID for this account type)
//   4. Implements other traits Anchor needs
#[account]

// "pub" = public.
// "struct" = defines a structure (like a class with only data, no methods).
// "Counter" = the name of this struct (capitalize by convention).
pub struct Counter {

    // "pub" = this field is public (accessible from outside the struct).
    // "count" = the field name.
    // ":" separates name from type.
    // "u64" = the type. "u" means unsigned (no negative numbers), "64" means
    //   64 bits. Can hold values from 0 to 18,446,744,073,709,551,615.
    //   Other options: u8, u16, u32, u128, i8, i16, i32, i64, i128 (i = signed)
    pub count: u64,

// "}" closes the struct definition.
}

// ============================================================================
// ACCOUNTS STRUCT: Initialize
// Defines which accounts the "initialize" instruction requires
// ============================================================================

// "#[derive(Accounts)]" is a derive macro. "derive" auto-implements traits
// (interfaces) for your struct. "Accounts" is an Anchor trait that:
//   1. Validates all accounts are correct
//   2. Deserializes account data
//   3. Checks all security constraints you specified
#[derive(Accounts)]

// "pub struct Initialize" = public struct named Initialize.
//
// "<'info>" is a "lifetime parameter". This is advanced Rust concept, but
// basically:
//   - Rust tracks how long references are valid to prevent bugs
//   - "'info" (apostrophe + name) declares a lifetime called "info"
//   - This tells Rust "all references in this struct live for the same
//     duration"
//   - Anchor requires this on all account structs - just include it and
//     don't worry about it for now
pub struct Initialize<'info> {

    // "#[account(...)]" is an attribute that specifies constraints/rules
    // for this account. Multiple constraints are separated by commas.
    #[account(
        // "init" = this account doesn't exist yet; create it!
        // Anchor will call the System Program to allocate space on-chain.
        init,

        // "payer = user" = the "user" account (defined below) will pay
        // the SOL required for rent. Storing data on Solana costs money!
        payer = user,

        // "space = 8 + 8" = allocate 16 bytes of space for this account.
        //   - First 8: Anchor's "discriminator" (identifies the account type)
        //   - Second 8: our "count" field (u64 = 8 bytes)
        // You must calculate this yourself! Formula:
        //   8 (discriminator) + size of all your fields
        space = 8 + 8
    )]

    // "pub counter" = public field named "counter".
    // ": Account<'info, Counter>" = the type is "Account" with two params:
    //   - "'info" = the lifetime (same as the struct's lifetime)
    //   - "Counter" = the data type this account holds (our struct above)
    // "Account" is an Anchor type that wraps a Solana account and provides:
    //   - Automatic deserialization of account data
    //   - Validation that the account is owned by this program
    //   - Type-safe access to the data
    pub counter: Account<'info, Counter>,

    // Another account constraint attribute.
    // "mut" = this account must be mutable (its data or SOL balance will
    // change). The user is paying rent, so their balance decreases.
    #[account(mut)]

    // "Signer<'info>" is a special Anchor type meaning:
    //   - This account must have SIGNED the transaction
    //   - Proves the owner of this account authorized this action
    //   - If someone tries to call initialize without the user's signature,
    //     the transaction will fail
    pub user: Signer<'info>,

    // No attribute here - no special constraints needed.
    //
    // "Program<'info, System>" is an Anchor type for program accounts:
    //   - "Program" = this is a program, not a data account
    //   - "System" = specifically the System Program
    //
    // The System Program is Solana's built-in program that can:
    //   - Create new accounts
    //   - Transfer SOL
    //   - Allocate space
    //
    // We need it here because "init" creates a new account, and only the
    // System Program can do that.
    pub system_program: Program<'info, System>,

// "}" closes the struct.
}

// ============================================================================
// ACCOUNTS STRUCT: Increment
// Defines which accounts the "increment" instruction requires
// ============================================================================

#[derive(Accounts)]
pub struct Increment<'info> {

    // Just one account needed: the counter we want to increment.
    //
    // "mut" = mutable, because we're changing the count value.
    //
    // No "init" because the account already exists.
    // No "payer" because we're not creating anything.
    // No "system_program" because we're not creating anything.
    #[account(mut)]
    pub counter: Account<'info, Counter>,

    // Notice: no Signer required! This means ANYONE can increment the
    // counter, not just the person who created it. If you wanted to
    // restrict this, you'd add an "authority" pubkey to the Counter struct
    // and a Signer here that must match it.
}
