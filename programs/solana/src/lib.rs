use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("AckSBrz9UShZJdzKBXAeUP9jX7AX9MPdR9RGrvzGwoXb");

#[program]
pub mod solana {
    use super::*;

    pub fn store_crime(ctx: Context<StoreCrime>, outcome: String, reward: u64) -> Result<()> {
        let crime: &mut Account<Crime> = &mut ctx.accounts.crime;
        let author: &Signer = &ctx.accounts.author;        
        let clock: Clock = Clock::get().unwrap();

        if outcome.len() > 10 {
            return Err(ErrorCode::OutcomeTooLong.into());
        }

        if reward > 100 {
            return Err(ErrorCode::RewardTooHigh.into());
        }

        crime.author = *author.key;
        crime.timestamp = clock.unix_timestamp;
        crime.outcome = outcome.to_string();
        crime.reward = reward;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StoreCrime<'info>{
    #[account(init, payer = author, space = Crime::LEN)]
    pub crime: Account<'info, Crime>,
    #[account(mut)]
    pub author: Signer<'info>,
    /// CHECK: This is a system program account, and its address is known and fixed.
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[account]
pub struct Crime {
    pub author: Pubkey,
    pub timestamp: i64,
    pub outcome: String,
    pub reward: u64
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const STRING_OUTCOME: usize = 10 * 4; // 10 chars max.
const U64_LENGTH: usize = 8;

impl Crime {
    const LEN: usize = DISCRIMINATOR_LENGTH + PUBLIC_KEY_LENGTH + TIMESTAMP_LENGTH + STRING_LENGTH_PREFIX + STRING_OUTCOME + U64_LENGTH;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Outcome too long")]
    OutcomeTooLong,
    #[msg("Reward too high")]
    RewardTooHigh
}