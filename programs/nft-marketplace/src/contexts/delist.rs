use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{state::Listing, Marketplace};

#[derive(Accounts)]
pub struct Delist<'info> {

    #[account(mut)]
    pub seller: Signer<'info>,

    pub seller_nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = seller_nft_mint,
        associated_token::authority = seller
    )]
   pub  seller_ata_for_nft: InterfaceAccount<'info, TokenAccount>,

   #[account(
    seeds = [b"marketplace", marketplace.name.as_bytes()],
    bump = marketplace.bump
)]
pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        close = seller,
        seeds = [b"listing", marketplace.key().as_ref(), seller_nft_mint.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        associated_token::mint = seller_nft_mint,
        associated_token::authority = listing,
        associated_token::token_program = token_program
    )]
    pub nft_vault: InterfaceAccount<'info, TokenAccount>,

   
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Delist<'info> {

    pub fn withdraw_nft(&mut self) -> Result<()> {

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.nft_vault.to_account_info(),
            mint: self.seller_nft_mint.to_account_info(),
            to: self.seller_ata_for_nft.to_account_info(),
            authority: self.listing.to_account_info(),
        };
        
          let marketplace = self.marketplace.key();
        let seller = self.seller.key();
        let seller_nft_mint = self.seller_nft_mint.key();
        let listing_seeds: &[&[u8]] = &[
            b"listing",
            marketplace.as_ref(),
            seller.as_ref(),
            seller_nft_mint.as_ref(),
            &[self.listing.bump],
        ];
        let signers_seeds = &[listing_seeds];
        
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signers_seeds);

        transfer_checked(cpi_ctx, 1, 0)?;

        //todo -> verify this closing vault
    
        // let cpi_program = self.token_program.to_account_info();
        // let cpi_accounts = CloseAccount {
        //     account: self.nft_vault.to_account_info(),
        //     destination: self.seller.to_account_info(),
        //     authority: self.listing.to_account_info(),
        // };
        // let signer_seeds = &[
        //     b"listing",
        //     self.marketplace.key().as_ref(),
        //     self.seller_nft_mint.key().as_ref(),
        //     &[self.listing.bump],
        // ];
        // let signers_seeds = &[&signer_seeds[..]];

        // let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // close_account(cpi_ctx)?;

        Ok(())
    }
}
