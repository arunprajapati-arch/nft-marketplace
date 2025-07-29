use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{error::MarketplaceError, state::{Listing, Marketplace},};


#[derive(Accounts)]
pub struct Purchase<'info> {

    pub nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [
            b"listing",
            marketplace.key().as_ref(),
            seller.key().as_ref(),
            nft_mint.key().as_ref(),
        ],
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,

   
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = listing,
    )]
    pub nft_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub buyer_ata_for_nft: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Purchase<'info> {
        
    pub fn transfer_nft(&mut self) -> Result<()> {

        let marketplace = self.marketplace.key();
        let seller = self.seller.key();
        let nft_mint = self.nft_mint.key();
        let listing_seeds: &[&[u8]] = &[
            b"listing",
            marketplace.as_ref(),
            seller.as_ref(),
            nft_mint.as_ref(),
            &[self.listing.bump],
        ];
        let signers_seeds = &[listing_seeds];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            TransferChecked {
                from: self.nft_vault.to_account_info(),
                mint: self.nft_mint.to_account_info(),
                to: self.buyer_ata_for_nft.to_account_info(),
                authority: self.listing.to_account_info(),
            },
            signers_seeds,
        );

       
        transfer_checked(cpi_ctx, 1, self.nft_mint.decimals)
    }

    pub fn transfer_sol(&mut self) -> Result<()> {
        let fee_lamports = (self.marketplace.fee as u64)
            .checked_mul(self.listing.price)
            .ok_or(MarketplaceError::MathOverflow)?
            .checked_div(100)
            .ok_or(MarketplaceError::MathOverflow)?;

        let seller_lamports = self
            .listing
            .price
            .checked_sub(fee_lamports)
            .ok_or(MarketplaceError::MathOverflow)?;

        let treasury_transfer_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.buyer.to_account_info(),
                to: self.treasury.to_account_info(),
            },
        );
        transfer(treasury_transfer_ctx, fee_lamports)?;

        let seller_transfer_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.buyer.to_account_info(),
                to: self.seller.to_account_info(),
            },
        );
        transfer(seller_transfer_ctx, seller_lamports)?;

        Ok(())
    }
}