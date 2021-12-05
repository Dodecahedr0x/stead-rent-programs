#![cfg_attr(feature = "no-entrypoint", allow(dead_code))]

use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;

use instructions::*;

declare_id!("TrXDop6spRAwHDsSpvY51PxHkvZXKGNYC6bygXZLNC2");

#[program]
mod stead_rent {
    use super::*;

    /// Initializes the state of the program
    pub fn initialize_state(
        ctx: Context<InitializeState>,
        bump: u8,
        fee_earner: Pubkey,
        fee_amount: u16,
    ) -> ProgramResult {
        instructions::init_state::handler(ctx, bump, fee_earner, fee_amount)
    }

    /// Sets the state of the program
    pub fn set_state(ctx: Context<SetState>, fee_earner: Pubkey, fee_amount: u16) -> ProgramResult {
        instructions::set_state::handler(ctx, fee_earner, fee_amount)
    }

    /// Creates an exhibition, held by a token
    pub fn initialize_exhibition(
        ctx: Context<InitializeExhibition>,
        bumps: InitExhibitionBumpSeeds,
        renter_fee: u16,
    ) -> ProgramResult {
        instructions::init_exhibition::handler(ctx, bumps, renter_fee)
    }

    /// Prevents the artist from depositing more items
    pub fn cancel_exhibition(ctx: Context<CancelExhibition>) -> ProgramResult {
        instructions::cancel_exhibition::handler(ctx)
    }

    /// Closes the exhibition to enable opening a new one with another artist
    pub fn close_exhibition(ctx: Context<CloseExhibition>) -> ProgramResult {
        instructions::close_exhibition::handler(ctx)
    }

    /// Lets the exhibitor deposit tokens in the exhibition
    pub fn deposit_token(
        ctx: Context<DepositToken>,
        bumps: DepositTokenSeedBumps,
        price: u64,
    ) -> ProgramResult {
        instructions::deposit_token::handler(ctx, bumps, price)
    }

    /// Lets the exhibitor withdraw tokens from the exhibition
    pub fn withdraw_token(
        ctx: Context<WithdrawToken>,
        _bumps: WithdrawTokenSeedBumps,
    ) -> ProgramResult {
        instructions::withdraw_token::handler(ctx)
    }

    /// Lets anyone buy one of the token deposited in the exhibition
    pub fn buy_token(ctx: Context<BuyToken>, bumps: BuyTokenSeedBumps) -> ProgramResult {
        instructions::buy_token::handler(ctx, bumps)
    }
}

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
    pub const LEN: usize = 40 + 3;
}

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

    /// The volume of pieces sold since the start of the exhibition
    pub total_volume: u64,

    /// The status of the exhibition
    pub status: ExhibitionStatus,

    /// Bumps used to sign PDA
    pub bumps: InitExhibitionBumpSeeds,
}

impl Exhibition {
    pub const LEN: usize = 3 * 40 + 2 + 8 + 8 + 3;
}

/// An item in the exhibition
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