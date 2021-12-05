use anchor_lang::prelude::*;

use crate::{State};
use crate::errors::*;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeState<'info> {
    /// The exhibition
    #[account(
        init,
        seeds = [b"state"],
        bump = bump,
        payer = payer,
        space = State::LEN
    )]
    pub state: Account<'info, State>,

    /// The wallet paying the transaction
    pub payer: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

/// Creates an exhibition and 
pub fn handler(
    ctx: Context<InitializeState>,
    bump: u8,
    fee_earner: Pubkey,
    fee_amount: u16
) -> ProgramResult {
    if fee_amount > 10000 {
        return Err(ErrorCode::FeeOutOfRangeError.into());
    }
    
    let state = &mut ctx.accounts.state;
    state.bump = bump;
    state.fee_earner = fee_earner;
    state.fee_amount = fee_amount;

    msg!("State initialized");

    Ok(())
}
