use anchor_lang::prelude::*;
use anchor_spl::token_interface::{ Mint, TokenInterface };

use crate::state::Marketplace;
use crate::state::Listing;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    // It's mutable because the admin is a signer and we need to change the admin
    #[account(mut)]
    admin: Signer<'info>,

    // The marketplace PDA we're creating is a new account, so we need to pay for it
    #[account(
        init,
        space = Marketplace::INIT_SPACE,
        payer = admin,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump
    )]
    marketplace: Account<'info, Marketplace>,

    // The rewards mint is for the rewards token, which is a new mint
    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        mint::authority = marketplace,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump
    )]
    rewards_mint: InterfaceAccount<'info, Mint>,

    // The treasury is for the marketplace fees, which is a new account
    #[account(seeds = [b"treasury", marketplace.key().as_ref()], bump)]
    treasury: SystemAccount<'info>,

    // The system program is required for creating new accounts
    system_program: Program<'info, System>,

    // The token program is required for creating new mints
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    // This is the entrypoint for the Initialize instruction
    // It's called from the client with the name of the marketplace and the fee
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        require!(name.len() > 0 && name.len() < 33, MarketplaceError::NameTooLong);
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            name,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            rewards_bump: bumps.rewards_mint,
        });

        Ok(())
    }
}
