use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub recipient: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub withdrawal: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub interval: u64,
}

impl Escrow {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 8 + 8 + 8;

    pub fn amount_unlocked(&self, now: u64) -> Option<u64> {
        // minimum from now and end time
        let time = if now < self.end_time {
            now
        } else {
            self.end_time
        };

        // end_at - start_at, difference of timestamps in second
        let duration = self.end_time.checked_sub(self.start_time)?;

        // amount * interval / duration
        let interval_amount = self
            .amount
            .checked_mul(self.interval)?
            .checked_div(duration)?;

        // (time - self.start_at) / self.interval + 1, current and passed intervals are unlocked
        let nr_intervals = time
            .checked_sub(self.start_time)?
            .checked_div(self.interval)?
            .checked_add(1)?;

        // nr_intervals * interval_amount - self.withdrawal
        nr_intervals
            .checked_mul(interval_amount)?
            .checked_sub(self.withdrawal)
    }
}
