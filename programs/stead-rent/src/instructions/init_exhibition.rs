use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{Exhibition, ExhibitionStatus, State};
use crate::errors::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitExhibitionBumpSeeds {
    pub exhibition: u8,
    pub escrow: u8,
    pub exhibition_token: u8
}

#[derive(Accounts)]
#[instruction(bumps: InitExhibitionBumpSeeds)]
pub struct InitializeExhibition<'info> {
    /// The global state
    #[account(
        seeds = [b"state"],
        bump = state.bump
    )]
    pub state: Account<'info, State>,
    
    /// The exhibition that will be created
    #[account(
        init,
        payer = payer,
        space = Exhibition::LEN,
        seeds = [
            b"exhibition".as_ref(),
            exhibition_token_mint.key().as_ref()
        ],
        bump = bumps.exhibition
    )]
    pub exhibition: Account<'info, Exhibition>,
    
    /// The account owning stored NFTs
    #[account(
        seeds = [
            b"escrow".as_ref(),
            exhibition_token_mint.key().as_ref()
        ],
        bump = bumps.escrow
    )]
    pub escrow: AccountInfo<'info>,

    /// The mint of the exposition token
    #[account(mut)]
    pub exhibition_token_mint: AccountInfo<'info>,

    /// The account that will hold the exhibition token
    #[account(
        init,
        seeds = [
            b"token_account".as_ref(),
            exhibition_token_mint.key().as_ref()
        ],
        bump = bumps.exhibition_token,
        token::mint = exhibition_token_mint,
        token::authority = escrow,
        payer = payer
    )]
    pub exhibition_token_account: Account<'info, TokenAccount>,

    /// The owner of the exhibition token
    pub renter: Signer<'info>,

    /// The renter's account holding the exhibition token
    #[account(
        mut,
        constraint = 
            renter_account.mint == exhibition_token_mint.key() &&
            renter_account.owner == renter.key() &&
            renter_account.amount == 1
    )]
    pub renter_account: Account<'info, TokenAccount>,

    /// The exhibitor
    pub exhibitor: AccountInfo<'info>,

    /// The account paying the transaction
    pub payer: AccountInfo<'info>,

    /// The program for interacting with the token.
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExhibition<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.renter_account.to_account_info(),
                to: self.exhibition_token_account.to_account_info(),
                authority: self.renter.to_account_info()
            }
        )
    }
}

/// Creates an exhibition and 
pub fn handler(
    ctx: Context<InitializeExhibition>,
    bumps: InitExhibitionBumpSeeds,
    renter_fee: u16
) -> ProgramResult {
    if renter_fee > 10000 - ctx.accounts.state.fee_amount {
        return Err(ErrorCode::FeeOutOfRangeError.into());
    }

    let exhibition = &mut ctx.accounts.exhibition;

    exhibition.renter = ctx.accounts.renter.key();
    exhibition.property = ctx.accounts.renter_account.mint.key();
    exhibition.renter_fee = renter_fee;
    exhibition.exhibitor = ctx.accounts.exhibitor.key();
    exhibition.status = ExhibitionStatus::Active;
    exhibition.bumps = bumps;

    token::transfer(ctx.accounts.transfer_context(), 1)?;

    msg!("Exhibition opened");

    Ok(())
}
