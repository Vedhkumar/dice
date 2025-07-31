use anchor_lang::prelude::*;

use crate::{error::ErrorCode, Config, CONFIG_SEED, VAULT_SEED};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [VAULT_SEED.as_bytes()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [CONFIG_SEED.as_bytes()],
        bump
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn process_initialize(
        &mut self,
        entry_fee: u64,
        lowest_value: u64,
        highest_value: u64,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        require!(entry_fee > 0, ErrorCode::InvalidEntryFee);
        require!(lowest_value < highest_value, ErrorCode::InvalidRange);
        self.config.set_inner(Config {
            authority: self.authority.key(),
            entry_fee,
            lowest_value,
            highest_value,
            vault: self.vault.key(),
            bump: bumps.config,
        });
        Ok(())
    }
}
