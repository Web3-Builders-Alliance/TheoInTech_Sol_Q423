use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{ TokenAccount, Mint, TokenInterface, TransferChecked, transfer_checked },
    metadata::{ Metadata, MetadataAccount, MasterEditionAccount },
    associated_token::AssociatedToken,
    token::{ transfer, Transfer },
};

pub use crate::state::{ Creator, ProjectDAO, Milestone, Reward };

pub use crate::errors::{ ProjectDAOError, MilestoneError, RewardError };

#[derive(Accounts)]
#[instruction(identifier: String, milestone_idx: u8, reward_idx: u8)]
pub struct Create<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    /*
     * DAOCre-8 Vault
     */
    #[account(mut, seeds = [b"daocre-8"], bump)]
    vault: SystemAccount<'info>,
    /*
     * Creator
     */
    #[account(
        init_if_needed,
        payer = signer,
        space = Creator::INIT_SPACE,
        seeds = [b"creator", signer.key().as_ref()],
        bump
    )]
    creator: Account<'info, Creator>,
    /*
     * NFT - qq: Do I really need all these to mint and transfer the NFT to the creator once project DAO is created?
     */
    creator_mint: InterfaceAccount<'info, Mint>,
    collection_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        associated_token::authority = signer,
        associated_token::mint = creator_mint
    )]
    creator_ata: Account<'info, Creator>,
    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), creator_mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() ==
        collection_mint.key().as_ref()
    )]
    metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    master_edition: Account<'info, MasterEditionAccount>,
    metadata_program: Program<'info, Metadata>,
    associated_token_program: Program<'info, AssociatedToken>,
    /*
     * ProjectDAO
     * qq: If I want the ProjectDAO to be token-gated using the NFT given to the creator, should I simply add it as a constraint or as part of a seed? What's the difference?
     */
    #[account(
        init,
        payer = signer,
        space = ProjectDAO::INIT_SPACE,
        seeds = [b"projectdao", creator.key().as_ref(), identifier.as_str().as_bytes()],
        bump
    )]
    // Consider using Box if we reach the limit
    project_dao: Account<'info, ProjectDao>,
    #[account(
        init,
        payer = signer,
        space = Milestone::INIT_SPACE,
        seeds = [b"milestone", milestone_idx.as_str().as_bytes(), project_dao.key().as_ref()],
        bump
    )]
    milestone: Account<'info, Milestone>,
    #[account(seeds = [b"treasury", project_dao.key().as_ref()], bump)]
    treasury: SystemAccount<'info>,
    #[account(
        init,
        payer = signer,
        space = Reward::INIT_SPACE,
        seeds = [b"reward", reward_idx.as_str().as_bytes(), project_dao.key().as_ref()],
        bump
    )]
    reward: Account<'info, Reward>,
    /*
     * System
     */
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> Create<'info> {
    pub fn initialize_creator(&mut self) -> Result<()> {
        self.creator.set_inner(Creator {
            signer: self.signer.key(),
            project_dao_count: 1,
            bump: bumps.creator,
            project_dao_bump: bumps.project_dao,
        });

        Ok(())
    }

    pub fn create_project_dao(
        &mut self,
        identifier: String,
        funding_goal: u64,
        initial_capital: u64,
        funding_start_date: u64,
        funding_end_date: u64,
        detail_metadata: String,
        updates_metadata: String,
        bumps: &InitializeBumps
    ) -> Result<()> {
        require!(identifier.len() > 0 && identifier.len() < 33, ProjectDAOError::IdentifierTooLong);
        require!(
            !ProjectDAO::exists(&self.project_dao.key(), ProjectDAOError::IdentifierAlreadyExists)
        );

        self.project_dao.set_inner(ProjectDAO {
            admin: self.signer.key(),
            identifier,
            funding_goal,
            initial_capital,
            funding_start_date,
            funding_end_date,
            detail_metadata,
            updates_metadata,
            bump: bumps.project_dao,
            treasury_bump: bumps.treasury,
            milestone_bump: bumps.milestone,
            reward_bump: bumps.reward,
        });

        Ok(())
    }

    pub fn create_milestone(
        &mut self,
        milestone_idx: u8,
        fund_disbursed: u64,
        receiver: PubKey,
        deadline: u64,
        milestone_metadata: String,
        bumps: &InitializeBumps
    ) -> Result<()> {
        require!(milestone_idx > 0, MilestoneError::MilestoneCannotBeEmpty);

        self.milestone.set_inner(Milestone {
            project: self.project_dao.key(),
            fund_disbursed,
            receiver: self.signer.key(),
            deadline,
            milestone_metadata,
            bump: bumps.milestone,
            milestone_poll_bump: bumps.milestone_poll,
        });

        Ok(())
    }

    pub fn create_reward(
        &mut self,
        reward_idx: u8,
        price: u64,
        reward_metadata: String,
        bumps: &InitializeBumps
    ) -> Result<()> {
        equire!(reward_idx > 0, RewardError::RewardCannotBeEmpty);

        self.reward.set_inner(Reward {
            project: self.project_dao.key(),
            price,
            reward_metadata,
            bump: bumps.reward,
        });

        Ok(())
    }

    pub fn deposit_fee(&mut self, fee: u64) -> Result<()> {
        let accounts = Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(cpi_ctx, fee)
    }

    // Finish this instruction from metaplex(?)
    pub fn mint_nft(&mut self) -> Result<()> {
        Ok(())
    }
}
