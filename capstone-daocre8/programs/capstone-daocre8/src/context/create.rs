use anchor_lang::{ prelude::*, system_program::{ Transfer, transfer } };
use mpl_bubblegum::{
    accounts::{ MerkleTree, TreeConfig },
    instructions::{ MintV1CpiBuilder },
    programs::{ MPL_BUBBLEGUM_ID, SPL_ACCOUNT_COMPRESSION_ID, SPL_NOOP_ID },
    types::MetadataArgs,
};

pub use crate::state::{ Creator, ProjectDAO, Milestone, Reward };
pub use crate::errors::{ ProjectDAOError, MilestoneError, RewardError };

#[derive(Accounts)]
#[instruction(project_dao_idx: String, milestone_idx: String, reward_idx: String)]
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
     * ProjectDAO
     * qq: If I want the ProjectDAO to be token-gated using the NFT given to the creator, should I simply add it as a constraint or as part of a seed? What's the difference?
     */
    #[account(
        init,
        payer = signer,
        space = ProjectDAO::INIT_SPACE,
        seeds = [b"projectdao", creator.key().as_ref(), project_dao_idx.as_str().as_bytes()],
        bump
    )]
    project_dao: Account<'info, ProjectDAO>, // Consider using Box if we reach the limit
    #[account(
        init,
        payer = signer,
        space = Milestone::INIT_SPACE,
        seeds = [b"milestone", milestone_idx.as_str().as_bytes(), project_dao.key().as_ref()],
        bump
    )]
    milestone: Account<'info, Milestone>,
    /*
     * cNFT
     * qq - I don't have an idea what I'm doing here
     */
    // #[account(address = SPL_ACCOUNT_COMPRESSION_ID)]
    // compression_program: AccountInfo<'info>,
    // #[account(address = SPL_NOOP_ID)]
    // log_wrapper: AccountInfo<'info>,
    // #[account(address = MPL_BUBBLEGUM_ID)]
    // bubblegum_program: AccountInfo<'info>,
    // #[account(address = "CNr5kujH2UPbLVNQ6wZ3NkfEwiQJuR9BD2GYTGWAj1i4")]
    // merkle_tree: AccountInfo<'info>,
    // // TODO: Make sure to change this
    // #[account(address = "CNr5kujH2UPbLVNQ6wZ3NkfEwiQJuR9BD2GYTGWAj1i4")]
    // tree_config: AccountInfo<'info>,
    /*
     * Funding
     */
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
}

impl<'info> Create<'info> {
    pub fn initialize_creator(&mut self, bumps: &CreateBumps) -> Result<()> {
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
        project_dao_idx: String,
        funding_goal: u64,
        initial_capital: u64,
        funding_start_date: u64,
        funding_end_date: u64,
        detail_metadata: String,
        updates_metadata: String,
        bumps: &CreateBumps
    ) -> Result<()> {
        require!(
            project_dao_idx.len() > 0 && project_dao_idx.len() < 33,
            ProjectDAOError::IdentifierTooLong
        );
        require!(
            // TODO: check to see if the project DAO already exists using the project_dao_idx
            true,
            ProjectDAOError::IdentifierAlreadyExists
        );

        self.project_dao.set_inner(ProjectDAO {
            admin: self.signer.key(),
            project_dao_idx,
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
        milestone_idx: String,
        fund_disbursed: u64,
        deadline: u64,
        milestone_metadata: String,
        bumps: &CreateBumps
    ) -> Result<()> {
        require!(milestone_idx.len() > 0, MilestoneError::MilestoneCannotBeEmpty);

        self.milestone.set_inner(Milestone {
            milestone_idx,
            project: self.project_dao.key(),
            fund_disbursed,
            receiver: self.signer.key(),
            deadline,
            milestone_metadata,
            bump: bumps.milestone,
        });

        Ok(())
    }

    pub fn create_reward(
        &mut self,
        reward_idx: String,
        price: u64,
        reward_metadata: String,
        vote_weight: u8,
        bumps: &CreateBumps
    ) -> Result<()> {
        require!(reward_idx.len() > 0, RewardError::RewardCannotBeEmpty);

        self.reward.set_inner(Reward {
            reward_idx,
            project: self.project_dao.key(),
            price,
            vote_weight,
            number_of_backers: 0,
            reward_metadata,
            bump: bumps.reward,
        });

        Ok(())
    }

    pub fn deposit_fee(&mut self, fee: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(cpi_ctx, fee)
    }

    pub fn initialize_project_dao_nft(&mut self) -> Result<()> {
        todo!(
            "Initialize a new merkle tree for the different rewards so once backers fund the project, they will also mint the nft"
        )
    }

    // pub fn mint_nft(&mut self, metadata: MetadataArgs) -> Result<()> {
    //     let signer_seeds: &[&[&[u8]]] = todo!("Get the correct signer_seeds for this");

    //     todo!("Fill in the correct values for the fields");
    //     // instruction accounts
    //     let cpi_mint = MintV1CpiBuilder::new(&self.bubblegum_program.to_account_info())
    //         .compression_program(&self.compression_program.to_account_info())
    //         .leaf_delegate(&self.signer.to_account_info())
    //         .leaf_owner(&self.signer.to_account_info())
    //         .log_wrapper(&self.log_wrapper.to_account_info())
    //         .merkle_tree(&self.merkle_tree.to_account_info())
    //         .payer(&self.signer.to_account_info())
    //         .system_program(&self.system_program.to_account_info())
    //         .tree_config(&self.tree_config.to_account_info())
    //         .metadata(metadata);

    //     // performs the CPI
    //     cpi_mint.invoke_signed(signer_seeds);
    //     Ok(())
    // }
}
