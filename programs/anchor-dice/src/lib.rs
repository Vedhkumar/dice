pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("As9EPNMqUb24aHr7MYjRhJ37Bdy4uSbDztdVbNAYVz7M");

#[program]
pub mod anchor_dice {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        entry_fee: u64,
        lowest_value: u64,
        highest_value: u64,
    ) -> Result<()> {
        ctx.accounts
            .process_initialize(entry_fee, lowest_value, highest_value, &ctx.bumps)
    }
}
