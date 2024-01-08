use anchor_lang::prelude::*;

declare_id!("AwjhMae1WRzopt1CvbCxnHbwys11yqdFTiJtNgTRgzJR");

#[program]
pub mod capstone_daocre8 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
