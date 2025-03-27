use anchor_lang::prelude::*;

declare_id!("5nGCT258vXxWFWzQV3PxDZLWKE2hXmyoWe7A2QhRzBvY");

#[program]
pub mod pumpswap_smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
