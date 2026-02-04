use anchor_lang::prelude::*;

declare_id!("2LUoJnKc5maGZYcyqMcGk2WwdKwTygxwRP7uqBUu6h6y");

#[program]
pub mod counter {
    use super::*;

    // Creates a new counter account, initialized to 0
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter initialized! Current count: {}", counter.count);
        Ok(())
    }

    // Increments the counter by 1
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}

// Account structure that stores the counter value
#[account]
pub struct Counter {
    pub count: u64,
}

// Accounts required for the initialize instruction
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,                          // Create a new account
        payer = user,                  // User pays for account creation
        space = 8 + 8                  // 8 bytes for discriminator + 8 bytes for u64
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,           // The user creating the counter

    pub system_program: Program<'info, System>,
}

// Accounts required for the increment instruction
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]                    // Mark as mutable since we're changing it
    pub counter: Account<'info, Counter>,
}
