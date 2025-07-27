#![allow(unexpected_cfgs, deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
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
}
