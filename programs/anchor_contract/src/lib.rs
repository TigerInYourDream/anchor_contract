use anchor_lang::prelude::*;

declare_id!("GCAamUbe319XUwGFaTzSKhBYW7yU6y1D5TQy1HY4Y4Bb");

#[program]
pub mod anchor_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
