use anchor_lang::prelude::*;
use jupiter_cpi::cpi;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod swap {



    use super::*;

    pub fn make_swap(ctx: Context<StartSwap>, in_amount: Option<u64>, minimum_out_amount: u64, platform_fee_bps: u8) -> Result<()> {
        let cpi_program = ctx.accounts.jupiter_program.to_account_info();
        let cpi_accounts = cpi::accounts::MercurialExchange {
            swap_program: ctx.accounts.authority.to_account_info(),
            swap_state: ctx.accounts.authority.to_account_info(),
            token_program: ctx.accounts.authority.to_account_info(),
            pool_authority: ctx.accounts.authority.to_account_info(),
            user_transfer_authority: ctx.accounts.authority.to_account_info(),
            source_token_account: ctx.accounts.authority.to_account_info(),
            destination_token_account: ctx.accounts.authority.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        cpi::mercurial_exchange(cpi_ctx, in_amount, minimum_out_amount, platform_fee_bps)
    }
}

#[derive(Accounts)]
pub struct StartSwap<'info> {
    pub jupiter_program: Program<'info, jupiter_cpi::program::Jupiter>,
    #[account(mut)]
    pub authority: Signer<'info>,
}
