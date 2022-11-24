use anchor_lang::prelude::*;

#[error_code]
pub enum SwapError {
    #[msg("Only Profitable arbitrage opportunities will be executed")]
    NotProfitableOpportunity,
    #[msg("Only Valid Exchange Ids are allowed")]
    InvalidExchange
}