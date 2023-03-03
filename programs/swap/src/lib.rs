use anchor_lang::prelude::*;

declare_id!("HG4rN1XSQQrEMjfZCiS88WkBEQaLbNyTtPopuHsgc1gq");

#[program]
pub mod swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
