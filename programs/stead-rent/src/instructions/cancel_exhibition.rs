use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct CancelExhibition<'info> {
    /// The exhibition
    #[account(mut)]
    pub exhibition: Account<'info, Exhibition>,

    /// The onwer of the propertry
    #[account(constraint = renter.key() == exhibition.renter)]
    pub renter: Signer<'info>,
}

/// Buys a token from the exhibition and split revenues
pub fn handler(
    ctx: Context<CancelExhibition>
) -> ProgramResult {
    ctx.accounts.exhibition.status = ExhibitionStatus::Cancelled;

    msg!("Cancelled");

    Ok(())
}
