use anchor_lang::prelude::*;

declare_id!("GUsNRWuXCaf5s24AcisHGtxjgVWAJRuwx9P9YsNK1r84");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
