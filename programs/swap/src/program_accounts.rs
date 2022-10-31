
// {Program, account, Signer, UncheckedAccount, CpiContext, ToAccountInfo, Accounts}
use anchor_lang::prelude::*;
use jupiter_cpi::cpi;

#[derive(Accounts)]
pub struct MercurialRaydium<'info> {
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
}

impl<'info> MercurialRaydium<'info> {
    pub fn mercurial_ctx(&self) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::MercurialExchange<'info>> {
        let cpi_program = self.jupiter_program.to_account_info();
        let cpi_accounts = cpi::accounts::MercurialExchange {
            swap_program: self.swap_program.to_account_info(),
            swap_state: self.swap_state.to_account_info(),
            pool_authority: self.pool_authority.to_account_info(),
            token_program: self.token_program.to_account_info(),
            source_token_account: self.source_token.to_account_info(),
            destination_token_account: self.destination_token.to_account_info(),
            user_transfer_authority: self.authority.to_account_info(),
        };
        
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn raydium_ctx(&self) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::RaydiumSwapV2<'info>> {
        // work out the correct account mapping here
        let cpi_program = self.jupiter_program.to_account_info();
        let cpi_accounts = cpi::accounts::RaydiumSwapV2 {
            amm_id: self.swap_program.to_account_info(),
            swap_program: self.swap_program.to_account_info(),
            amm_open_orders: self.swap_state.to_account_info(),
            token_program: self.token_program.to_account_info(),
            amm_authority: self.pool_authority.to_account_info(),
            pool_coin_token_account: self.source_token.to_account_info(),
            pool_pc_token_account: self.destination_token.to_account_info(),
            serum_program_id: self.swap_state.to_account_info(),
            serum_market: self.pool_authority.to_account_info(),
            serum_event_queue: self.swap_state.to_account_info(),
            serum_bids: self.swap_state.to_account_info(),
            serum_asks: self.swap_state.to_account_info(),
            serum_coin_vault_account: self.pool_authority.to_account_info(),
            serum_pc_vault_account: self.swap_state.to_account_info(),
            serum_vault_signer: self.swap_state.to_account_info(),
            user_source_owner: self.authority.to_account_info(),
            user_source_token_account: self.source_token.to_account_info(),
            user_destination_token_account: self.destination_token.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

}