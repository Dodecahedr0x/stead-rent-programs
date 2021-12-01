use anchor_lang::prelude::*;

/// The global state of the program
#[account]
pub struct State {
    /// The bump used to generate this PDA
    pub bump: u8, 

    /// The wallet to which fees are given
    pub fee_earner: Pubkey,

    /// The portion of the sale which goes to the fee earner
    /// Denominated in basis points
    pub fee_amount: u16,
}

impl State {
    pub const LEN: usize = 40 + 4;
}