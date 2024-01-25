use anchor_lang::prelude::*;

declare_id!("AwjhMae1WRzopt1CvbCxnHbwys11yqdFTiJtNgTRgzJR");

pub mod state;
pub mod context;
pub mod errors;

pub use context::*;
pub use errors::*;

#[program]
pub mod capstone_daocre8 {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        ctx.accounts.initialize_creator(&ctx.bumps)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
