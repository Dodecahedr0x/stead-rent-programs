use anchor_lang::prelude::*;

use crate::{State};
use crate::errors::*;

#[derive(Accounts)]
pub struct SetState<'info> {
    /// The exhibition
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump,
    )]
    pub state: Account<'info, State>,

    #[account(constraint = owner.key() == state.fee_earner)]
    pub owner: Signer<'info>
}

/// Creates an exhibition and 
pub fn handler(
    ctx: Context<SetState>,
    fee_earner: Pubkey,
    fee_amount: u16
) -> ProgramResult {
    if fee_amount > 10000 {
        return Err(ErrorCode::FeeOutOfRangeError.into());
    }
    
    let state = &mut ctx.accounts.state;
    state.fee_earner = fee_earner;
    state.fee_amount = fee_amount;

    msg!("State set");

    Ok(())
}
