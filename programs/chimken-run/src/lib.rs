use anchor_lang::prelude::*;

declare_id!("FQZhPRFBoCn1ZvvMBUb14sdUFHw8g3GTFLbQjP5vuw1a");

pub mod contexts;
pub mod states;

pub use contexts::*;

#[program]
pub mod chimken_run {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, entry_price: u64, duration: u64) -> Result<()> {
        ctx.accounts.init(entry_price, duration, &ctx.bumps)
    }

    pub fn join(ctx: Context<Join>, entry_price: u64) -> Result<()> {
        ctx.accounts.join()?;
        ctx.accounts.realloc()?;
        ctx.accounts.pay(entry_price)
    }

    pub fn finish(ctx: Context<Finish>, score: u32) -> Result<()> {
        ctx.accounts.finish(score)
    }

    pub fn winner(ctx: Context<Winner>) -> Result<()> {
        ctx.accounts.winner()
    }
}
