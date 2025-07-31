use anchor_lang::{prelude::*, solana_program::system_instruction::transfer};

use crate::{bet, error::ErrorCode, BetAccount, Config, BET_SEED, CONFIG_SEED};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED.as_bytes()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [BET_SEED.as_bytes(), player.key().as_ref(), id.to_le_bytes().as_ref()
        ],
        bump = bet_account.bump,
        close = player,
    )]
    pub bet_account: Account<'info, BetAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    pub fn process_bet(&mut self, bet_amount: u64, guess_number: u64) -> Result<()> {
        Ok(())
    }
}
