use anchor_lang::prelude::*;

// Rust Macro - argument(program_public_id)
declare_id!("GNkWQQUyJMk1LeZnX9wAiagN7fj88EoXHiD5k754vCAK");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter =  &mut ctx.accounts.counter;
        counter.bump = ctx.bumps.counter; //store bump seed in 'Counter' account
        msg!("Counter account created! Current count: {}", counter.count);
        msg!("Counter bump: {}", counter.bump);
        // msg!("Greetings from: {:?}", ctx.program_id);

        Ok(())
    }
    
    pub fn increment(ctx: Context<Increment>)-> Result<()>{
        let counter = &mut ctx.accounts.counter;
        msg!("Previous Count: {}",counter.count);

        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter increased!, Current count: {}", counter.count);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(mut)]
     pub user:Signer<'info>,

    #[account(
        init,
        seeds = [b"counter"],
        bump,
        payer = user, //the wallet/user paying for the creation of the counter account
        space = 8 + Counter::INIT_SPACE // space allocated to the new account (8 byte discriminator + 8 byte for u64 )
    )]
    
    pub counter:Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}


// Account needed by increment instruction
#[derive(Accounts)]
pub struct Increment<'info>{
    #[account(
        mut,
        seeds =[b"counter"],
        bump = counter.bump,
    )]

    pub counter: Account<'info, Counter>,
}
// Define Structure of the 'Counter' account
#[account]
#[derive(InitSpace)]
pub struct Counter{
    pub count: u64, //Count value type
    pub bump: u8 //1 byte for pda
}
 