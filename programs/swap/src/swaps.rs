use std::usize;

use jupiter_cpi::cpi;
use jupiter_cpi::typedefs::Side;
use anchor_lang::prelude::*;

use crate::errors::SwapError;


pub struct ExchangeInfo{
    name: String,
    accounts: u8
}

pub fn get_exchange_info(idx: u8) -> ExchangeInfo {
    match idx {
        0 => ExchangeInfo{
            name: "Orca".to_string(),
            // minus signer and token program
            accounts: 9
        },
        1 => ExchangeInfo{
            name: "Aldrin".to_string(),
            // minus signer and token program
            accounts: 9
        },
        _ => {
            panic!("Invalid Exchange Id");
            ExchangeInfo{
                name: "".to_string(),
                accounts: 0
            };
        }
    }
}


pub fn make_generic_swap<'info>(idx: u8, offset: u8, in_amount: Option<u64>, minimum_out_amount: u64, jupiter_program: &Program<'info, jupiter_cpi::program::Jupiter>, wallet_authority: &Signer<'info>, token_program: &UncheckedAccount<'info>, accounts: &[AccountInfo<'info>]) -> Result<()> {
    match idx {
        0 => {
            let cpi_program = jupiter_program.to_account_info();
            let start = offset as usize;
            let end = start + get_exchange_info(idx).accounts as usize;
            let accounts = &accounts[start..end];
            let cpi_accounts = cpi::accounts::AldrinSwap {
                swap_program: accounts[0].clone(),
                pool: accounts[1].clone(),
                pool_signer: accounts[2].clone(),
                pool_mint: accounts[3].clone(),
                base_token_vault: accounts[4].clone(),
                quote_token_vault: accounts[5].clone(),
                fee_pool_token_account: accounts[6].clone(),
                user_base_token_account: accounts[7].clone(),
                user_quote_token_account: accounts[8].clone(),
                
                token_program: token_program.to_account_info(),
                wallet_authority: wallet_authority.to_account_info(),
            };
            cpi::aldrin_swap(CpiContext::new(cpi_program, cpi_accounts), in_amount, minimum_out_amount, Side::Bid,0)
        },
        1 => {
            let cpi_program = jupiter_program.to_account_info();
            let start = offset as usize;
            let end = start + get_exchange_info(idx).accounts as usize;
            let accounts = &accounts[start..end];
            let cpi_accounts = cpi::accounts::TokenSwap {
                token_swap_program: accounts[0].clone(),
                swap: accounts[1].clone(),
                authority: accounts[2].clone(),
                source: accounts[3].clone(),
                swap_source: accounts[4].clone(),
                swap_destination: accounts[5].clone(),
                destination: accounts[6].clone(),
                pool_mint: accounts[7].clone(),
                pool_fee: accounts[8].clone(),
                
                token_program: token_program.to_account_info(),
                user_transfer_authority: wallet_authority.to_account_info(),
            };
            cpi::token_swap(CpiContext::new(cpi_program, cpi_accounts), in_amount, minimum_out_amount, 0)
        },
        _ => {err!(SwapError::InvalidExchange)}
    }
}