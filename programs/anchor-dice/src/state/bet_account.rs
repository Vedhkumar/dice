use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BetAccount {
    pub id: u64,
    pub player: Pubkey,
    pub amount: u64,
    pub guess_number: u64,
    pub bump: u8,
}
