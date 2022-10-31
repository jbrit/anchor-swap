mod program_accounts;
use anchor_lang::prelude::*;
use jupiter_cpi::cpi;
mod amm;

declare_id!("4NDjSubeiiiAg6Y11crMVAjmqNLcHWiJvo9bk9G8Jemn");

#[program]
pub mod swap {
    use super::*;

    pub fn make_swap<'info>(ctx: Context<'_, '_, '_, 'info, StartSwap<'info>>, in_amount: Option<u64>, minimum_out_amount: u64) -> Result<()> {
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
        let mut cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        cpi_ctx = cpi_ctx.with_remaining_accounts(ctx.remaining_accounts.to_vec());
        // setting a 0 platfprm_fee_bps
        cpi::mercurial_exchange(cpi_ctx, in_amount, minimum_out_amount, 0)
    }

    pub fn mercurial_raydium<'info>(ctx: Context<'_, '_, '_, 'info, program_accounts::MercurialRaydium<'info>>) -> Result<()> {
        let mut x = [0.0,0.0] as [f64; 2];
        let mut y = [0.0,0.0] as [f64; 2];
        let in_amount = amm::get_optimal_input( x, y);
        let out_amount = 0 as u64;
        // cpi::mercurial_exchange(ctx.accounts.mercurial_ctx(), in_amount, minimum_out_amount, 0)
        // cpi::raydium_swap_v2(ctx.accounts.raydium_ctx(), in_amount, minimum_out_amount, 0)
        Ok(())
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
    // source and destination token accounts need to be mutable by the CPI program
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub source_token: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub destination_token: UncheckedAccount<'info>,
    // expecting first 3 accounts to be passed in for mercurial exchange
}