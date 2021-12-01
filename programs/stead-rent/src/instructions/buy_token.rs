use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BuyTokenSeedBumps {
    item: u8,
    token_account: u8
}

#[derive(Accounts)]
#[instruction(bumps: BuyTokenSeedBumps)]
pub struct BuyToken<'info> {
    /// The exhibition
    #[account(mut)]
    pub exhibition: Account<'info, Exhibition>,

    /// The owner of the token being bought
    #[account(mut)]
    pub exhibitor: AccountInfo<'info>,

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

    /// The mint of the token being bought
    #[account(mut)]
    pub deposited_token_mint: AccountInfo<'info>,

    /// The account that holds the token being bought
    #[account(
        mut,
        seeds = [
            b"token_account".as_ref(),
            deposited_token_mint.key().as_ref()
        ],
        bump = bumps.token_account
    )]
    pub deposited_token_account: Account<'info, TokenAccount>,

    /// The buyer
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// The buyer's account that will hold the token being bought
    #[account(
        mut,
        constraint =
            buyer_account.mint == deposited_token_mint.key() &&
            buyer_account.owner == buyer.key()
    )]
    pub buyer_account: Account<'info, TokenAccount>,

    /// The program for interacting with the token.
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

/// Creates an exhibition and 
pub fn handler(
    ctx: Context<BuyToken>,
    _bump: BuyTokenSeedBumps
) -> ProgramResult {
    msg!("Buying...");
    let exhibition = &mut ctx.accounts.exhibition;
    exhibition.n_pieces -= 1;

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.buyer.key,
        &ctx.accounts.exhibitor.key,
        ctx.accounts.exhibition_item.price,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[ctx.accounts.buyer.to_account_info(), ctx.accounts.exhibitor.clone()],
    )?;

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
            to: ctx.accounts.buyer_account.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info()
        },
        signer,
    );

    token::transfer(context, 1)?;

    msg!("Token bought");

    Ok(())
}
