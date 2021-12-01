use anchor_lang::prelude::*;

/// Rental property that will serve as an art gallery
#[account]
pub struct ExhibitionItem {
    /// The exhibition this item is a part of
    pub exhibition: Pubkey,

    /// The mint of the item
    pub mint: Pubkey,

    /// The price defined by the exhibitor
    pub price: u64,
}

impl ExhibitionItem {
    pub const LEN: usize = 40 + 40 + 8;
}