#![allow(unexpected_cfgs, deprecated)]
pub mod constants;
pub mod error;
pub mod contexts;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use contexts::*;
pub use state::*;

declare_id!("CsqKJ7DphT4c8FiaYSHss7E8ZoAKTpY7qhwQPvAudSrf");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn init_marketplace(
        context: Context<InitializeMarketplace>,
        name: String,
        fee: u16,
    ) -> Result<()> {
        context.accounts.init(name, fee, &context.bumps)
    }

    pub fn list_nft(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()
    }


    pub fn delist_nft(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()
    }

    pub fn purchase_nft(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.transfer_nft()?;
        ctx.accounts.transfer_sol()?;
        Ok(())
    }
}
