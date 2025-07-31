use anchor_lang::{prelude::*, solana_program::system_instruction::transfer};

use crate::{bet, error::ErrorCode, BetAccount, Config, BET_SEED, CONFIG_SEED};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Bet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED.as_bytes()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = player,
        space = 8 + BetAccount::INIT_SPACE,
        seeds = [BET_SEED.as_bytes(), player.key().as_ref(), id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub bet_account: Account<'info, BetAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> Bet<'info> {
    pub fn process_bet(
        &mut self,
        id: u64,
        bet_amount: u64,
        guess_number: u64,
        bumps: &BetBumps,
    ) -> Result<()> {
        require!(
            bet_amount >= self.config.entry_fee,
            ErrorCode::InvalidEntryFee
        );
        require!(
            guess_number >= self.config.lowest_value && guess_number <= self.config.highest_value,
            ErrorCode::InvalidRange
        );

        transfer(self.player.key, &self.config.vault, bet_amount);
        self.bet_account.set_inner(BetAccount {
            id,
            player: self.player.key(),
            amount: bet_amount,
            guess_number,
            bump: bumps.bet_account,
        });
        Ok(())
    }
}
