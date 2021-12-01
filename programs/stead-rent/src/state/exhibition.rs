use anchor_lang::prelude::*;

use crate::InitExhibitionBumpSeeds;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ExhibitionStatus {
    Active,
    Cancelled
}

/// Rental property that will serve as an art gallery
#[account]
pub struct Exhibition {
    /// The owner property where the exhibition takes place
    pub renter: Pubkey,
    
    /// The property hosting the exhibition
    pub property: Pubkey,

    /// The fee earned by the renter on each sell
    pub renter_fee: u16,

    /// The owner of tokens to be displayed in the exhibition
    pub exhibitor: Pubkey,

    /// The number of pieces currently in the exhibition
    pub n_pieces: u64,

    /// The status of the exhibition
    pub status: ExhibitionStatus,

    /// Bumps used to sign PDA
    pub bumps: InitExhibitionBumpSeeds,
}

impl Exhibition {
    pub const LEN: usize = 3 * 40 + 2 + 8 + 8 + 3;
}