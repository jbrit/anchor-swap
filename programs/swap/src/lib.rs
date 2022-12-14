mod program_accounts;
use anchor_lang::prelude::*;
use jupiter_cpi::cpi;
use jupiter_cpi::typedefs::Side;
mod amm;
mod errors;
mod swaps;
use anchor_spl::token::TokenAccount;

declare_id!("BeTX9jpRTDKPUCH3rNuNwERQNp3fHxoE1beFFGeVaicG");

#[program]
pub mod swap {
    use super::*;

    pub fn make_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, StartSwap<'info>>,
        in_amount: Option<u64>,
        minimum_out_amount: u64,
    ) -> Result<()> {
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

    pub fn mercurial_raydium<'info>(
        ctx: Context<'_, '_, '_, 'info, MercurialRaydium<'info>>,
    ) -> Result<()> {
        // TODO: get the actual reserves here or as an argument
        // can't check remaining account reserves
        let mut mercurial_reserves = [0.0, 0.0] as [f64; 2];
        let mut raydium_reserves = [
            ctx.accounts.pool_coin_token_account.amount as f64,
            ctx.accounts.pool_pc_token_account.amount as f64,
        ] as [f64; 2];

        let in_amount = amm::get_optimal_input(&mut mercurial_reserves, &mut raydium_reserves);
        // convert to Option u64
        let mercurial_in_amount = Some(in_amount as u64);
        let mercurial_out_amount =
            amm::get_amount_out(in_amount, &mut mercurial_reserves, &mut raydium_reserves) as u64;
        // out amount from mercurial exchange is in amount for raydium
        let raydium_in_amount = Some(mercurial_out_amount);
        let raydium_out_amount = amm::get_amount_out(
            raydium_in_amount.unwrap() as f64,
            &mut mercurial_reserves,
            &mut raydium_reserves,
        ) as u64;

        match raydium_out_amount > mercurial_in_amount.unwrap() {
            true => {
                match cpi::mercurial_exchange(
                    ctx.accounts
                        .mercurial_ctx()
                        .with_remaining_accounts(ctx.remaining_accounts.to_vec()),
                    mercurial_in_amount,
                    mercurial_out_amount,
                    0,
                ) {
                    Ok(_) => cpi::raydium_swap_v2(
                        ctx.accounts.raydium_ctx(),
                        raydium_in_amount,
                        raydium_out_amount,
                        0,
                    ),
                    Err(e) => Err(e),
                }
            }
            false => err!(errors::SwapError::NotProfitableOpportunity),
        }
    }

    pub fn aldrin_orca<'info>(ctx: Context<'_, '_, '_, 'info, AldrinOrca<'info>>) -> Result<()> {
        let mut aldrin_reserves = [
            ctx.accounts.quote_token_vault.amount as f64,
            ctx.accounts.swap_source.amount as f64,
        ] as [f64; 2];
        let mut orca_reserves = [
            ctx.accounts.base_token_vault.amount as f64,
            ctx.accounts.swap_destination.amount as f64,
        ] as [f64; 2];
        msg!("Aldrin Reserves : {:?}", aldrin_reserves);
        msg!("Orca Reserves : {:?}", orca_reserves);
        let in_amount = amm::get_optimal_input(&mut aldrin_reserves, &mut orca_reserves);
        msg!("OI : {}", in_amount);
        // convert to Option u64
        let aldrin_in_amount = Some(in_amount as u64);
        let aldrin_out_amount =
            amm::get_amount_out(in_amount, &mut aldrin_reserves, &mut orca_reserves) as u64;
        // out amount from aldrin exchange is in amount for orca
        let orca_in_amount = Some(aldrin_out_amount);
        let orca_out_amount = amm::get_amount_out(
            orca_in_amount.unwrap() as f64,
            &mut aldrin_reserves,
            &mut orca_reserves,
        ) as u64;
        msg!("Orca Out Amount : {}", orca_out_amount);
        msg!("Aldrin OI : {}", aldrin_in_amount.unwrap());
        match orca_out_amount > aldrin_in_amount.unwrap() {
            true => {
                match cpi::aldrin_swap(
                    ctx.accounts.aldrin_ctx(),
                    aldrin_in_amount,
                    aldrin_out_amount,
                    Side::Bid,
                    0,
                ) {
                    Ok(_) => {
                        cpi::token_swap(ctx.accounts.orca_ctx(), orca_in_amount, orca_out_amount, 0)
                    }
                    Err(e) => Err(e),
                }
            }
            false => err!(errors::SwapError::NotProfitableOpportunity),
        }
    }

    pub fn generic_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, GenericSwap<'info>>,
        exchange_id: u8,
        in_amount: Option<u64>,
        minimum_out_amount: u64,
    ) -> Result<()> {
        swaps::make_generic_swap(
            exchange_id,
            0,
            in_amount,
            minimum_out_amount,
            &ctx.accounts.jupiter_program,
            &ctx.accounts.wallet_authority,
            &ctx.accounts.token_program,
            ctx.remaining_accounts,
        )
    }

    pub fn generic_aldrin_orca<'info>(
        ctx: Context<'_, '_, '_, 'info, GenericSwap<'info>>,
    ) -> Result<()> {
        let exchanges = [1, 0] as [u8; 2];
        let mut aldrin = swaps::get_exchange_info(exchanges[0], 0, ctx.remaining_accounts);
        let mut orca =
            swaps::get_exchange_info(exchanges[1], aldrin.account_count, ctx.remaining_accounts);
        let aldrin_in_amount =
            Some(amm::get_optimal_input(&mut aldrin.reserves, &mut orca.reserves) as u64);
        let aldrin_out_amount = amm::get_amount_out(
            aldrin_in_amount.unwrap() as f64,
            &mut aldrin.reserves,
            &mut orca.reserves,
        ) as u64;
        let orca_in_amount = Some(aldrin_out_amount);
        let orca_out_amount = amm::get_amount_out(
            orca_in_amount.unwrap() as f64,
            &mut aldrin.reserves,
            &mut orca.reserves,
        ) as u64;

        swaps::make_generic_swap(
            exchanges[0],
            0,
            aldrin_in_amount,
            aldrin_out_amount,
            &ctx.accounts.jupiter_program,
            &ctx.accounts.wallet_authority,
            &ctx.accounts.token_program,
            ctx.remaining_accounts,
        )?;

        swaps::make_generic_swap(
            exchanges[1],
            aldrin.account_count,
            orca_in_amount,
            orca_out_amount,
            &ctx.accounts.jupiter_program,
            &ctx.accounts.wallet_authority,
            &ctx.accounts.token_program,
            ctx.remaining_accounts,
        )
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

// Path: programs/swap/src/program_accounts.rs (couldn't find a way to import this properly)
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
    #[account(mut)]
    pub source_token: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub destination_token: Box<Account<'info, TokenAccount>>,

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
    pub serum_program_id: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub serum_vault_signer: UncheckedAccount<'info>,
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

    // token accounts
    #[account(mut)]
    pub serum_coin_vault_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub serum_pc_vault_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_coin_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_pc_token_account: Box<Account<'info, TokenAccount>>,
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

#[derive(Accounts)]
pub struct AldrinOrca<'info> {
    pub jupiter_program: Program<'info, jupiter_cpi::program::Jupiter>,
    pub wallet_authority: Signer<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub token_program: UncheckedAccount<'info>,

    // aldrin specific
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub swap_program: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub pool: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub pool_signer: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub aldrin_pool_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub base_token_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub quote_token_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub fee_pool_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_base_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_quote_token_account: Box<Account<'info, TokenAccount>>,

    // orca specific
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub token_swap_program: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub swap: UncheckedAccount<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub source: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub swap_source: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub swap_destination: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub destination: Box<Account<'info, TokenAccount>>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    #[account(mut)]
    pub orca_pool_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub pool_fee: Box<Account<'info, TokenAccount>>,
}

impl<'info> AldrinOrca<'info> {
    pub fn aldrin_ctx(&self) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::AldrinSwap<'info>> {
        let cpi_program = self.jupiter_program.to_account_info();
        let cpi_accounts = cpi::accounts::AldrinSwap {
            swap_program: self.swap_program.to_account_info(),
            pool: self.pool.to_account_info(),
            pool_signer: self.pool_signer.to_account_info(),
            pool_mint: self.aldrin_pool_mint.to_account_info(),
            base_token_vault: self.base_token_vault.to_account_info(),
            quote_token_vault: self.quote_token_vault.to_account_info(),
            fee_pool_token_account: self.fee_pool_token_account.to_account_info(),
            user_base_token_account: self.user_base_token_account.to_account_info(),
            user_quote_token_account: self.user_quote_token_account.to_account_info(),
            token_program: self.token_program.to_account_info(),
            wallet_authority: self.wallet_authority.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn orca_ctx(&self) -> CpiContext<'_, '_, '_, 'info, cpi::accounts::TokenSwap<'info>> {
        let cpi_program = self.jupiter_program.to_account_info();
        let cpi_accounts = cpi::accounts::TokenSwap {
            token_swap_program: self.token_swap_program.to_account_info(),
            swap: self.swap.to_account_info(),
            authority: self.authority.to_account_info(),
            source: self.source.to_account_info(),
            swap_source: self.swap_source.to_account_info(),
            swap_destination: self.swap_destination.to_account_info(),
            destination: self.destination.to_account_info(),
            pool_mint: self.orca_pool_mint.to_account_info(),
            pool_fee: self.pool_fee.to_account_info(),
            token_program: self.token_program.to_account_info(),
            user_transfer_authority: self.wallet_authority.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct GenericSwap<'info> {
    // these accounts are required for all swaps
    pub jupiter_program: Program<'info, jupiter_cpi::program::Jupiter>,
    pub wallet_authority: Signer<'info>,
    /// CHECK: we don't need to read it in our own program, just the cpi
    pub token_program: UncheckedAccount<'info>,
}
