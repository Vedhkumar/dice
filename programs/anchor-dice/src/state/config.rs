use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub entry_fee: u64,
    pub lowest_value: u64,
    pub highest_value: u64,
    pub vault: Pubkey,
    pub bump: u8,
}
