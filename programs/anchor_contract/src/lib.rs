use anchor_lang::prelude::*;
use anchor_spl::token::Token;

declare_id!("GCAamUbe319XUwGFaTzSKhBYW7yU6y1D5TQy1HY4Y4Bb");

#[program]
pub mod anchor_contract {

    use anchor_spl::token::{self, MintTo, Transfer};

    use super::*;

    pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
        let cpi_account = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_accout.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_account);
        token::mint_to(cpi_context, 10)?;
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>) -> Result<()>{
        let transfer_instruction = Transfer { 
            from: ctx.accounts.from.to_account_info(), 
            to: ctx.accounts.to.to_account_info(), 
            authority: ctx.accounts.singer.to_account_info(),
        }; 
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);
        token::transfer(cpi_ctx, 5)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct MintToken<'info> {
    /// CHECK: THIS IS NOT Dangrous, because we dont read or write form this account --just for compiler warning
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: THIS IS NOT Dangrous, because we dont read or write form this account
    #[account(mut)]
    pub token_accout: UncheckedAccount<'info>,
    /// CHECK: THIS IS NOT Dangrous, because we dont read or write form this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: THIS IS NOT Dangrous, because we dont read or write form this account
    #[account(mut)] 
    pub from: UncheckedAccount<'info>,
    /// CHECK: THIS IS NOT Dangrous, because we dont read or write form this account
    #[account(mut)] 
    pub to: UncheckedAccount<'info>,
    #[account(mut)]
    pub singer: Signer<'info>,
}
