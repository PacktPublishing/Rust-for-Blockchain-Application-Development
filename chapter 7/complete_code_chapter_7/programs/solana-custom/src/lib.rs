use anchor_lang::prelude::*;

declare_id!("2cazkdWKXoKgUUcPygLZ9Ty1iwL2qCdGXY9m7mZV1iDp");

#[program]
pub mod solana_custom {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
