# ============================================================================
# PYTHON "TRANSLATION" OF THE SOLANA COUNTER PROGRAM
# ============================================================================
#
# This is pseudocode to help you understand the Rust/Solana code.
# It will NOT actually run on Solana - it's just for learning!
#
# Read this side-by-side with lib.rs to see how concepts map.
# ============================================================================

# ----------------------------------------------------------------------------
# Rust:   use anchor_lang::prelude::*;
#
# Python: We'd import from a (fictional) anchor library.
#         In reality, no such Python library exists for on-chain programs.
# ----------------------------------------------------------------------------
from dataclasses import dataclass
from typing import Optional

# Pretend imports (these don't exist, just for illustration)
# from anchor_framework import Account, Signer, Program, Context, msg


# ----------------------------------------------------------------------------
# Rust:   declare_id!("2LUoJnKc5maGZYcyqMcGk2WwdKwTygxwRP7uqBUu6h6y");
#
# Python: Just a string constant. In Rust, the macro does compile-time
#         validation and embeds this into the program binary.
# ----------------------------------------------------------------------------
PROGRAM_ID = "2LUoJnKc5maGZYcyqMcGk2WwdKwTygxwRP7uqBUu6h6y"


# ----------------------------------------------------------------------------
# Rust:   #[account]
#         pub struct Counter {
#             pub count: u64,
#         }
#
# Python: A simple dataclass. But note the differences:
#         - Rust's #[account] macro adds serialization, discriminator, etc.
#         - Rust's u64 is exactly 8 bytes; Python's int is unlimited size
#         - Rust requires explicit "pub" for public fields
# ----------------------------------------------------------------------------
@dataclass
class Counter:
    count: int = 0  # u64 in Rust (0 to 18,446,744,073,709,551,615)


# ----------------------------------------------------------------------------
# Rust:   #[derive(Accounts)]
#         pub struct Initialize<'info> {
#             #[account(init, payer = user, space = 8 + 8)]
#             pub counter: Account<'info, Counter>,
#             #[account(mut)]
#             pub user: Signer<'info>,
#             pub system_program: Program<'info, System>,
#         }
#
# Python: A class holding references to accounts. But note:
#         - No equivalent to 'info lifetime (Rust memory safety)
#         - No equivalent to #[account(init, payer, space)] constraints
#         - No equivalent to Signer (cryptographic verification)
#         - No equivalent to Program type validation
# ----------------------------------------------------------------------------
@dataclass
class InitializeAccounts:
    counter: Optional[Counter] = None       # Account to create (init)
    user: Optional[str] = None              # Signer's pubkey (pays rent)
    system_program: Optional[str] = None    # Needed to create accounts

    # THINGS PYTHON CAN'T EXPRESS:
    # - "init" means Solana will CREATE this account on-chain
    # - "payer = user" means user's SOL balance decreases to pay rent
    # - "space = 8 + 8" means allocate exactly 16 bytes
    # - Signer means this person CRYPTOGRAPHICALLY SIGNED the transaction
    # - The system_program MUST be the real System Program (validated)


# ----------------------------------------------------------------------------
# Rust:   #[derive(Accounts)]
#         pub struct Increment<'info> {
#             #[account(mut)]
#             pub counter: Account<'info, Counter>,
#         }
#
# Python: Much simpler - just needs the counter.
#         - "mut" in Rust means we'll modify this account
#         - No Signer = anyone can call this instruction!
# ----------------------------------------------------------------------------
@dataclass
class IncrementAccounts:
    counter: Optional[Counter] = None  # The counter to increment (must be mut)


# ----------------------------------------------------------------------------
# Rust:   #[program]
#         pub mod counter {
#             use super::*;
#
#             pub fn initialize(ctx: Context<Initialize>) -> Result<()> { ... }
#             pub fn increment(ctx: Context<Increment>) -> Result<()> { ... }
#         }
#
# Python: A class with methods. But note:
#         - Rust's #[program] macro generates the entire entry point,
#           instruction routing, serialization, and error handling
#         - Rust's Result<()> forces explicit error handling
#         - Rust's Context<T> validates accounts match the T struct
# ----------------------------------------------------------------------------
class CounterProgram:
    """
    The Solana program logic.

    KEY DIFFERENCE FROM PYTHON:
    This class has NO STATE. It's not like a Python object that holds data.
    All data comes from accounts passed into each method.

    Think of it as a collection of pure functions, not an object.
    """

    # ------------------------------------------------------------------------
    # Rust:   pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    #             let counter = &mut ctx.accounts.counter;
    #             counter.count = 0;
    #             msg!("Counter initialized! Current count: {}", counter.count);
    #             Ok(())
    #         }
    #
    # Python: A method that takes accounts and modifies them.
    #         - "&mut" in Rust = mutable borrow (compile-time safety)
    #         - "msg!" in Rust = logs to blockchain (visible in explorers)
    #         - "Ok(())" in Rust = explicit success return
    # ------------------------------------------------------------------------
    def initialize(self, accounts: InitializeAccounts) -> None:
        """
        Creates a new counter account and sets it to 0.

        WHAT SOLANA DOES THAT PYTHON CAN'T:
        1. Validates 'user' actually signed the transaction
        2. Creates the counter account on-chain (allocates 16 bytes)
        3. Deducts rent from user's SOL balance
        4. Writes the discriminator (8 bytes identifying account type)
        5. Serializes counter.count to bytes and stores on-chain
        """
        # Rust: let counter = &mut ctx.accounts.counter;
        # The "&mut" means we're borrowing it mutably - Rust tracks this
        # at compile time to prevent data races. Python has no equivalent.
        counter = accounts.counter

        # Rust: counter.count = 0;
        counter.count = 0

        # Rust: msg!("Counter initialized! Current count: {}", counter.count);
        # This logs to the Solana transaction logs, not stdout
        print(f"Counter initialized! Current count: {counter.count}")

        # Rust: Ok(())
        # Explicit success. If we wanted to fail, we'd return Err(...)
        # Python uses exceptions instead of Result types
        return None

    # ------------------------------------------------------------------------
    # Rust:   pub fn increment(ctx: Context<Increment>) -> Result<()> {
    #             let counter = &mut ctx.accounts.counter;
    #             counter.count += 1;
    #             msg!("Counter incremented! Current count: {}", counter.count);
    #             Ok(())
    #         }
    # ------------------------------------------------------------------------
    def increment(self, accounts: IncrementAccounts) -> None:
        """
        Adds 1 to the counter.

        NOTE: No signature required! Anyone can call this.
        In a real app, you might want to restrict who can increment.
        """
        # Rust: let counter = &mut ctx.accounts.counter;
        counter = accounts.counter

        # Rust: counter.count += 1;
        counter.count += 1

        # Rust: msg!("Counter incremented! ...");
        print(f"Counter incremented! Current count: {counter.count}")

        # Rust: Ok(())
        return None


# ============================================================================
# CONCEPTS WITH NO PYTHON EQUIVALENT
# ============================================================================
#
# 1. OWNERSHIP & BORROWING (&, &mut)
#    Rust tracks who "owns" data and who is "borrowing" it. Prevents bugs
#    like two threads modifying the same data. Python doesn't do this.
#
# 2. LIFETIMES ('info)
#    Rust tracks how long references are valid. The <'info> on structs
#    tells Rust "these references all live the same duration." Python
#    uses garbage collection instead - no manual lifetime tracking.
#
# 3. MACROS (#[program], #[account], msg!)
#    Rust macros generate code at compile time. They can inspect your
#    code and generate boilerplate. Python decorators are runtime-only
#    and much less powerful.
#
# 4. RESULT TYPE (Result<()>)
#    Rust forces you to handle errors. You can't ignore a Result - the
#    compiler will complain. Python lets exceptions propagate silently.
#
# 5. FIXED-SIZE INTEGERS (u8, u16, u32, u64, u128)
#    Rust integers have exact byte sizes. u64 is always 8 bytes.
#    Python's int grows to arbitrary size. This matters for serialization
#    and calculating account space.
#
# 6. DISCRIMINATOR
#    Anchor adds 8 bytes to identify account types. When the program
#    reads raw bytes from an account, it checks the discriminator to
#    ensure it's the right type. Python objects know their own type.
#
# 7. RENT & PAYER
#    Storing data on Solana costs SOL (rent). Someone must pay.
#    There's no equivalent in traditional programming - storage is "free."
#
# 8. SIGNER & CRYPTOGRAPHIC VERIFICATION
#    Solana verifies someone signed the transaction with their private key.
#    This proves authorization. Python has no built-in crypto verification.
#
# 9. ACCOUNT MODEL (State External to Program)
#    The biggest difference! Solana programs don't hold state - they
#    receive accounts, modify them, and Solana persists the changes.
#    It's like every function receives a database connection and tables.
#
# 10. ON-CHAIN DEPLOYMENT
#     This code runs on thousands of computers worldwide, in a consensus
#     protocol. Python runs on one computer. Totally different execution
#     model.
#
# ============================================================================


# ============================================================================
# HOW YOU'D USE THIS IN PYTHON (for illustration only)
# ============================================================================
if __name__ == "__main__":
    # Create the "program" (in Solana, this is deployed on-chain)
    program = CounterProgram()

    # Create accounts (in Solana, these are created on-chain and passed in)
    counter_account = Counter()

    init_accounts = InitializeAccounts(
        counter=counter_account,
        user="FakeUserPublicKey123",
        system_program="11111111111111111111111111111111"  # Real system program ID
    )

    # Call initialize (in Solana, this is a transaction sent to the network)
    program.initialize(init_accounts)
    # Output: Counter initialized! Current count: 0

    # Call increment
    inc_accounts = IncrementAccounts(counter=counter_account)
    program.increment(inc_accounts)
    # Output: Counter incremented! Current count: 1

    program.increment(inc_accounts)
    # Output: Counter incremented! Current count: 2

    print(f"\nFinal count: {counter_account.count}")
    # Output: Final count: 2
