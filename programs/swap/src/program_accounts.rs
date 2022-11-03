
// {Program, account, Signer, UncheckedAccount, CpiContext, ToAccountInfo, Accounts}
use anchor_lang::prelude::*;
use jupiter_cpi::cpi;

#[derive(Accounts)]
pub struct MercurialRaydium<'info> {
    pub jupiter_program: Program<'info, jupiter_cpi::program::Jupiter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub token_program: UncheckedAccount<'info>,

    // mercurial exchange

    /// CHECK: we don't need to read it in our own program, just the cpi
    pub swap_program: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub pool_authority: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub swap_state: UncheckedAccount<'info>,
    // source and destination token accounts need to be mutable by the CPI program
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub source_token: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub destination_token: UncheckedAccount<'info>,
    
    // raydium accounts

    /// CHECK: we don't need to read it in our own program, just the cpi
    pub raydium_swap_program: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub amm_id: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub pool_coin_token_account: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub pool_pc_token_account: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub serum_program_id: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub serum_market: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub serum_bids: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub serum_asks: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub serum_event_queue: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub serum_coin_vault_account: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub serum_pc_vault_account: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub serum_vault_signer: UncheckedAccount<'info>,
    
}

impl<'info> MercurialRaydium<'info> {
    pub fn mercurial_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::MercurialExchange<'info>> {
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

    pub fn raydium_ctx(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::RaydiumSwapV2<'info>> {
        let cpi_program = self.jupiter_program.to_account_info();
        let cpi_accounts = cpi::accounts::RaydiumSwapV2 {
            amm_id: self.amm_id.to_account_info(),
            swap_program: self.raydium_swap_program.to_account_info(),
            amm_open_orders: self.amm_open_orders.to_account_info(),
            token_program: self.token_program.to_account_info(),
            amm_authority: self.amm_authority.to_account_info(),
            pool_coin_token_account: self.pool_coin_token_account.to_account_info(),
            pool_pc_token_account: self.pool_pc_token_account.to_account_info(),
            serum_program_id: self.serum_program_id.to_account_info(),
            serum_market: self.serum_market.to_account_info(),
            serum_event_queue: self.serum_event_queue.to_account_info(),
            serum_bids: self.serum_bids.to_account_info(),
            serum_asks: self.serum_asks.to_account_info(),
            serum_coin_vault_account: self.serum_coin_vault_account.to_account_info(),
            serum_pc_vault_account: self.serum_pc_vault_account.to_account_info(),
            serum_vault_signer: self.serum_vault_signer.to_account_info(),
            user_source_owner: self.authority.to_account_info(),
            // swapping source for destination here
            user_source_token_account: self.destination_token.to_account_info(),
            user_destination_token_account: self.source_token.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
