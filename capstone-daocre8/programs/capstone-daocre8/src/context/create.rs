use anchor_lang::{ prelude::*, system_program::{ Transfer, transfer } };
use anchor_spl::{
    token_interface::{ Mint, TokenInterface },
    metadata::{ Metadata, MetadataAccount },
    associated_token::AssociatedToken,
    token::{ mint_to, MintTo },
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
    // /*
    //  * NFT - qq: Do I really need all these to mint and transfer the NFT to the creator once project DAO is created?
    //  */
    // creator_mint: InterfaceAccount<'info, Mint>,
    // collection_mint: InterfaceAccount<'info, Mint>,
    // #[account(
    //     init_if_needed,
    //     payer = signer,
    //     associated_token::authority = signer,
    //     associated_token::mint = creator_mint
    // )]
    // creator_ata: Account<'info, Creator>,
    // #[account(
    //     seeds = [b"metadata", metadata_program.key().as_ref(), creator_mint.key().as_ref()],
    //     seeds::program = metadata_program.key(),
    //     bump,
    //     constraint = collection_mint.as_ref().unwrap().key.as_ref() ==
    //     collection_mint.key().as_ref()
    // )]
    // metadata_account: Account<'info, MetadataAccount>,
    // metadata_program: Program<'info, Metadata>,
    // associated_token_program: Program<'info, AssociatedToken>,
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
    // Consider using Box if we reach the limit
    project_dao: Account<'info, ProjectDAO>,
    #[account(
        init,
        payer = signer,
        space = Milestone::INIT_SPACE,
        seeds = [b"milestone", milestone_idx.as_str().as_bytes(), project_dao.key().as_ref()],
        bump
    )]
    milestone: Account<'info, Milestone>,
    #[account(seeds = [b"treasury", project_dao.key().as_ref()], bump)]
    /*
     * Funding
     */
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
        bumps: &CreateBumps
    ) -> Result<()> {
        require!(reward_idx.len() > 0, RewardError::RewardCannotBeEmpty);

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
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(cpi_ctx, fee)
    }

    // pub fn mint_nft(&mut self) -> Result<()> {
    //     // Mint the NFT to the creator's ATA
    //     let mint_to_accounts = MintTo {
    //         mint: self.creator_mint.to_account_info(),
    //         to: self.creator_ata.to_account_info(),
    //         authority: self.signer.to_account_info(),
    //     };
    //     let mint_to_ctx = CpiContext::new(self.token_program.to_account_info(), mint_to_accounts);
    //     mint_to(mint_to_ctx, 1)?;

    //     // Constructing the parameters for creating NFT metadata
    //     let metadata_accounts = create_metadata_accounts_v3(
    //         self.metadata_program.key(), // Public key of the Metaplex Token Metadata program
    //         self.metadata_account.key(), // Public key of the account where metadata will be stored
    //         self.creator_mint.key(), // Mint address of the NFT being created
    //         self.signer.key(), // Public key of the entity (user) initiating this transaction
    //         self.signer.key(), // Account with authority to update metadata in the future
    //         self.signer.key(), // Account responsible for paying the transaction fees
    //         nft_metadata_title, // Title of the NFT (e.g., name of the artwork, project, etc.)
    //         "NFT_SYMBOL", // Symbol for the NFT (usually a short, unique identifier for the NFT or collection)
    //         nft_metadata_uri, // URI pointing to the off-chain metadata (e.g., JSON file with more details about the NFT)
    //         None, // Creators of the NFT (optional, can be multiple creators with their share of royalties)
    //         0, // Royalty percentage (in basis points, e.g., 500 for 5%)
    //         true, // Indicates whether the metadata can be updated in the future (true if yes)
    //         None, // Master edition account (None for standard NFT; used for limited edition prints)
    //         None, // Reference to the larger collection this NFT is a part of (if applicable)
    //         None // Usage permissions for the NFT (None if standard NFT with no special usage constraints)
    //     );

    //     // Invoking the CPI (Cross-Program Invocation) to create metadata
    //     invoke_signed(
    //         &metadata_accounts,
    //         &[
    //             self.metadata_account.to_account_info(), // Metadata account
    //             self.creator_mint.to_account_info(), // Mint account for the NFT
    //             self.signer.to_account_info(), // Signer of the transaction
    //             self.signer.to_account_info(), // Authority to update metadata
    //             self.signer.to_account_info(), // Account paying for the transaction
    //             self.rent.to_account_info(), // Rent sysvar account (used for rent exemption calculations)
    //             self.token_program.to_account_info(), // Token program account
    //             self.system_program.to_account_info(), // System program account
    //         ],
    //         &[&["DAOCRE-8".as_bytes(), &[self.bumps.creator_mint]]] // Seeds for generating the PDA used in this transaction
    //     )?;

    //     Ok(())
    // }
}
