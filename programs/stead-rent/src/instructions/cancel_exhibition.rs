use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{Exhibition, ExhibitionStatus};

#[derive(Accounts)]
pub struct CancelExhibition<'info> {
    /// The exhibition
    #[account(mut)]
    pub exhibition: Account<'info, Exhibition>,

    /// The owner of the propertry
    #[account(constraint = renter.key() == exhibition.renter)]
    pub renter: Signer<'info>,

    /// The exhibitor's account that will receive the token
    #[account(
        mut,
        constraint =
            renter_account.mint == deposited_token_mint.key() &&
            renter_account.owner == renter.key()
    )]
    pub renter_account: Account<'info, TokenAccount>,

    /// The account owning stored NFTs
    #[account(
        seeds = [
            b"escrow".as_ref(),
            exhibition.property.as_ref()
        ],
        bump = exhibition.bumps.escrow
    )]
    pub escrow: AccountInfo<'info>,

    /// The mint of the token being withdrawn
    #[account(mut)]
    pub deposited_token_mint: AccountInfo<'info>,

    /// The account that holds the token being withdrawn
    #[account(
        mut,
        seeds = [
            b"token_account".as_ref(),
            deposited_token_mint.key().as_ref()
        ],
        bump = exhibition.bumps.exhibition_token
    )]
    pub deposited_token_account: Account<'info, TokenAccount>,

    /// The program for interacting with the token.
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

/// Buys a token from the exhibition and split revenues
pub fn handler(
    ctx: Context<CancelExhibition>
) -> ProgramResult {
    ctx.accounts.exhibition.status = ExhibitionStatus::Cancelled;

    let seeds = &[
        b"escrow".as_ref(),
        ctx.accounts.exhibition.property.as_ref(),
        &[ctx.accounts.exhibition.bumps.escrow],
    ];
    let signer = &[&seeds[..]];

    let context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.deposited_token_account.to_account_info(),
            to: ctx.accounts.renter_account.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info()
        },
        signer,
    );

    token::transfer(context, 1)?;

    msg!("Cancelled");

    Ok(())
}
