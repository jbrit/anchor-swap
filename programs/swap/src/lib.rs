use anchor_lang::prelude::*;
use jupiter_cpi::cpi;

declare_id!("HKRgB1McMjZ7gUzR6saGvQwoJ1mXNfBSCzeLo6SJSaQ4");

#[program]
pub mod swap {
    use super::*;

    pub fn make_swap(ctx: Context<StartSwap>, in_amount: Option<u64>, minimum_out_amount: u64) -> Result<()> {
        let cpi_program = ctx.accounts.jupiter_program.to_account_info();
        let cpi_accounts = cpi::accounts::MercurialExchange {
            swap_program: ctx.accounts.swap_program.to_account_info(),
            swap_state: ctx.accounts.swap_state.to_account_info(),
            pool_authority: ctx.accounts.pool_authority.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            source_token_account: ctx.accounts.source_token.to_account_info(),
            destination_token_account: ctx.accounts.destination_token.to_account_info(),
            user_transfer_authority: ctx.accounts.authority.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // setting a 0 platfprm_fee_bps
        cpi::mercurial_exchange(cpi_ctx, in_amount, minimum_out_amount, 0)
    }
}

#[derive(Accounts)]
pub struct StartSwap<'info> {
    pub jupiter_program: Program<'info, jupiter_cpi::program::Jupiter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub swap_program: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub pool_authority: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub swap_state: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub token_program: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub source_token: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub destination_token: UncheckedAccount<'info>,
}
