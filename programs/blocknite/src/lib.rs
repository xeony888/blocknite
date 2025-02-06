use anchor_lang::prelude::*;

declare_id!("2tYcASh4cCNAnxAygN3f6cEze6udchSb6jPc5xP9RdAE");

#[program]
pub mod blocknite {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
