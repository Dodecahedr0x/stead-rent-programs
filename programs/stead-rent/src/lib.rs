#![cfg_attr(feature = "no-entrypoint", allow(dead_code))]

use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod stead_rent {
    use super::*;

    /// Creates an exhibition, held by a token
    pub fn init_exhibition(ctx: Context<InitializeExhibition>, bumps: InitExhibitionBumpSeeds) -> ProgramResult {
        instructions::init_exhibition::handler(ctx, bumps)
    }

    /// Lets the exhibitor deposit tokens in the exhibition
    pub fn deposit_token(ctx: Context<DepositToken>, bumps: DepositTokenSeedBumps, price: u64) -> ProgramResult {
        instructions::deposit_token::handler(ctx, bumps, price)
    }

    /// Lets anyone buy one of the token deposited in the exhibition
    pub fn buy_token(ctx: Context<BuyToken>, bumps: BuyTokenSeedBumps) -> ProgramResult {
        instructions::buy_token::handler(ctx, bumps)
    }
}