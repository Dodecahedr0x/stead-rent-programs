use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DepositTokenSeedBumps {
    item: u8,
    token_account: u8,
}

#[derive(Accounts)]
#[instruction(bumps: DepositTokenSeedBumps)]
pub struct DepositToken<'info> {
    /// The exhibition
    #[account(mut, has_one = exhibitor, constraint = exhibition.status == ExhibitionStatus::Active)]
    pub exhibition: Account<'info, Exhibition>,

    /// The item for sale in the exhibition
    #[account(
        init,
        payer = payer,
        space = ExhibitionItem::LEN,
        seeds = [
            b"item".as_ref(),
            exhibition.key().as_ref(),
            deposited_token_mint.key().as_ref()
        ],
        bump = bumps.item
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

    /// The mint of the deposited token
    #[account(mut)]
    pub deposited_token_mint: AccountInfo<'info>,

    /// The account that will hold the deposited token
    #[account(
        init,
        seeds = [
            b"token_account".as_ref(),
            deposited_token_mint.key().as_ref()
        ],
        bump = bumps.token_account,
        token::mint = deposited_token_mint,
        token::authority = escrow,
        payer = payer
    )]
    pub deposited_token_account: Account<'info, TokenAccount>,

    /// The owner of the deposited token
    pub exhibitor: Signer<'info>,

    /// The exhibitor's account holding the deposited token
    #[account(
        mut,
        constraint =
            exhibitor_account.mint == deposited_token_mint.key() &&
            exhibitor_account.owner == exhibitor.key() &&
            exhibitor_account.amount == 1
    )]
    pub exhibitor_account: Account<'info, TokenAccount>,

    /// The account paying the transaction
    pub payer: AccountInfo<'info>,

    /// The program for interacting with the token.
    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositToken<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.exhibitor_account.to_account_info(),
                to: self.deposited_token_account.to_account_info(),
                authority: self.exhibitor.to_account_info(),
            },
        )
    }
}

/// Creates an exhibition and
pub fn handler(
    ctx: Context<DepositToken>,
    _bump: DepositTokenSeedBumps,
    price: u64,
) -> ProgramResult {
    let exhibition = &mut ctx.accounts.exhibition;
    exhibition.n_pieces += 1;

    let item = &mut ctx.accounts.exhibition_item;
    item.exhibition = exhibition.key();
    item.mint = ctx.accounts.deposited_token_mint.key();
    item.price = price;

    token::transfer(ctx.accounts.transfer_context(), 1)?;

    msg!("Token deposited");

    Ok(())
}
