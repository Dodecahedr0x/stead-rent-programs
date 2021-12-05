use anchor_lang::prelude::*;

use crate::{Exhibition};

#[derive(Accounts)]
pub struct CloseExhibition<'info> {
    /// The exhibition
    #[account(mut, close = renter, constraint = exhibition.n_pieces == 0)]
    pub exhibition: Account<'info, Exhibition>,

    /// The onwer of the propertry
    #[account(mut, constraint = renter.key() == exhibition.renter)]
    pub renter: AccountInfo<'info>,
}

/// Closes the exhibition account.
/// This is needed to start an exhibition with another artist
pub fn handler(
    _ctx: Context<CloseExhibition>
) -> ProgramResult {
    msg!("Closed");

    Ok(())
}
