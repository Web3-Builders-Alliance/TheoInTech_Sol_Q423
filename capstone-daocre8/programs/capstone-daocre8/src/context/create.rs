use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{ TokenAccount, Mint, TokenInterface, TransferChecked, transfer_checked },
    metadata::{ Metadata, MetadataAccount, MasterEditionAccount },
    associated_token::AssociatedToken,
};

pub use crate::state::{ Creator, ProjectDAO, Milestones, Treasury, Rewards };

pub use crate::errors::ProjectDAOError;

#[derive(Accounts)]
#[instruction(identifier: String)]
pub struct Create<'info> {
    #[account(mut)]
    signer: Signer<'info>,
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
    project_dao: Account<'info, ProjectDao>,
    /*
     * qq: There will be one or more Milestones once the ProjectDAO is created. How do I initialize them?
     */
    #[account(
        init,
        payer = signer,
        space = Milestones::INIT_SPACE,
        seeds = [b"milestones", project_dao.key().as_ref()],
        bump
    )]
    milestones: Account<'info, Milestones>,
    /*
     * qq: Who should be the owner of the Treasury? It shouldn't be the DAOCre-8 team and the creator so that no one can withdraw the funds, and should just be controlled by the program.
     */
    #[account(seeds = [b"treasury", project_dao.key().as_ref(), bump], bump)]
    treasury: SystemAccount<'info>,
    /*
     * qq: There will be one or more Rewards once the ProjectDAO is created. How do I initialize them?
     */
    #[account(
        init,
        payer = signer,
        space = Rewards::INIT_SPACE,
        seeds = [b"rewards", project_dao.key().as_ref()],
        bump
    )]
    rewards: Account<'info, Rewards>,
    /*
     * System
     */
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> Create<'info> {
    pub fn create_project_dao(
        &mut self,
        // ProjectDAO
        identifier: String,
        funding_goal: u64,
        initial_capital: u64,
        funding_start_date: u64,
        funding_end_date: u64,
        detail_metadata: String,
        updates_metadata: String,
        fee: u16,
        // Milestones
        fund_disbursed: u64,
        receiver: PubKey,
        deadline: u64,
        milestones_metadata: String,
        // Treasury
        // Rewards
        bumps: &InitializeBumps
    ) -> Result<()> {
        // Validations
        require!(identifier.len() > 0 && identifier.len() < 33, ProjectDAOError::IdentifierTooLong);
        require!(
            !ProjectDAO::exists(&self.project_dao.key(), ProjectDAOError::IdentifierAlreadyExists)
        );

        // Initialize Creator, then ProjectDAO, Milestones, Treasury, and Rewards
        self.creator.set_inner(Creator {
            signer: self.signer.key(),
            project_dao_count: 1,
            bump: bumps.creator,
            project_dao_bump: bumps.project_dao,
        });
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
            milestones_bump: bumps.milestones,
            rewards_bump: bumps.rewards,
        });

        // qq: I need to initialize multiple Milestones when creating the ProjectDAO, how do I do it in the same instruction?
        // self.milestones.set_inner(Milestones {
        //     project: self.project_dao.key(),
        //     fund_disbursed,
        //     receiver: self.signer.key(),
        //     deadline,
        //     milestones_metadata,
        //     bump: bumps.milestones,
        //     milestone_polls_bump: bumps.milestone_polls,
        // });

        // Transfer the SOL fee to the DAOCre-8 Treasury wallet

        // Mint and Transfer DAOCre-8 NFT

        // qq: Is it still a good idea to have an instruction as huge as this? If not, how do I split it up?
    }
}
