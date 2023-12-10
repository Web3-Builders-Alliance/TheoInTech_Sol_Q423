use anchor_lang::prelude::*;

declare_id!("Aqz3Njf8qMpEx5KTU6y1bSogmRLtq5M8HhDJ4KpgNJDu");

pub mod contexts;
pub use contexts::*;

pub mod state;
use state::*;

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?;
        ctx.accounts.save_escrow(seed, receive, &ctx.bumps)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()?;
        ctx.accounts.close_vault()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw()?;
        ctx.accounts.close_vault()
    }
}
