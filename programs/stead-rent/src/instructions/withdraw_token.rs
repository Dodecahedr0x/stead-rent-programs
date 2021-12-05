use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{Exhibition, ExhibitionItem};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WithdrawTokenSeedBumps {
    token_account: u8
}

#[derive(Accounts)]
#[instruction(bumps: WithdrawTokenSeedBumps)]
pub struct WithdrawToken<'info> {
    /// The exhibition
    #[account(mut)]
    pub exhibition: Account<'info, Exhibition>,

    /// The exhibitor
    #[account(mut, constraint = exhibitor.key() == exhibition.exhibitor)]
    pub exhibitor: Signer<'info>,

    /// The exhibitor's account that will receive the token
    #[account(
        mut,
        constraint =
            exhibitor_account.mint == deposited_token_mint.key() &&
            exhibitor_account.owner == exhibitor.key()
    )]
    pub exhibitor_account: Account<'info, TokenAccount>,

    /// The item for sale in the exhibition
    #[account(
        mut,
        close = exhibitor,
        constraint = 
            exhibition_item.exhibition == exhibition.key() &&
            exhibition_item.mint == deposited_token_mint.key()
    )]
    pub exhibition_item: Account<'info, ExhibitionItem>,
    
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
        bump = bumps.token_account
    )]
    pub deposited_token_account: Account<'info, TokenAccount>,

    /// The program for interacting with the token.
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

/// Buys a token from the exhibition and split revenues
pub fn handler(
    ctx: Context<WithdrawToken>
) -> ProgramResult {
    let exhibition = &mut ctx.accounts.exhibition;
    exhibition.n_pieces -= 1;

    let seeds = &[
        b"escrow".as_ref(),
        exhibition.property.as_ref(),
        &[exhibition.bumps.escrow],
    ];
    let signer = &[&seeds[..]];

    let context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.deposited_token_account.to_account_info(),
            to: ctx.accounts.exhibitor_account.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info()
        },
        signer,
    );

    token::transfer(context, 1)?;

    msg!("Token withdrawn");

    Ok(())
}
