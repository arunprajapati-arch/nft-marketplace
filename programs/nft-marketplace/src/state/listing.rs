use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub nft_mint: Pubkey,
    pub price: u64,
    pub seller: Pubkey,
    pub bump: u8,
}