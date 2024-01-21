use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{ Metadata, MetadataAccount },
    token::{ self, Mint, Token, TokenAccount, MintTo, InitializeMint },
};
use metaplex_token_metadata::instruction::{ create_metadata_accounts_v3, MetadataInstruction };

#[derive(Accounts)]
pub struct InitializeCreatorNft<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(init, payer = signer, space = Mint::LEN)]
    collection_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"metadata", metadata_program.key().as_ref(), collection_mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump
    )]
    metadata_account: Account<'info, MetadataAccount>,
    metadata_program: Program<'info, Metadata>,
    /*
     * System
     */
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

impl<'info> InitializeCreatorNft<'info> {
    pub fn initialize_creator_nft(
        &mut self,
        bump_seed: u8,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String
    ) -> Result<()> {
        let seeds = &[&"DAOCRE-8".as_bytes(), &[bump_seed]]; // Seeds for generating a PDA for the mint
        let signer = &[&seeds[..]]; // Signer seeds for the CPI call

        let cpi_accounts = InitializeMint {
            mint: self.collection_mint.to_account_info(), // The mint account for the collection
            rent: self.rent.to_account_info(), // Rent sysvar account
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), // Token program account
            cpi_accounts,
            signer
        );
        // Initialize the mint, 0 decimals as it's an NFT
        token::initialize_mint(cpi_ctx, 0, &self.collection_mint.key(), None)?;

        // Create metadata for the collection
        let metadata_accounts = create_metadata_accounts_v3(
            self.metadata_program.key(), // Metaplex metadata program ID
            self.metadata_account.key(), // Metadata account for the NFT
            self.collection_mint.key(), // Mint account of the NFT
            self.signer.key(), // Account of the person initializing the NFT (could be an admin or creator)
            self.signer.key(), // Update authority account (responsible for updating the NFT metadata)
            self.signer.key(), // Payer for the metadata account creation (typically the user or the program itself)
            metadata_title, // Title of the NFT in the metadata
            metadata_symbol, // Symbol for the collection
            metadata_uri, // URI where the metadata JSON is stored (usually an IPFS or HTTP URL)
            None, // Creators of the NFT (optional, can be multiple)
            500, // Royalty basis points (e.g., 500 for 5%)
            true, // Update authority is the signer
            true, // Indicates if the metadata is mutable (true if it can be changed later)
            None, // Reference to the collection (optional, used for nested collections)
            None, // Uses (optional, can define how the NFT can be used)
            None // Collection details
        );

        invoke_signed(
            &metadata_accounts,
            &[
                self.metadata_account.clone(), // Metadata account
                self.collection_mint.to_account_info(), // Mint account
                self.signer.to_account_info(), // User account
                self.signer.to_account_info(), // Update authority account
                self.signer.to_account_info(), // Payer account
                self.rent.to_account_info(), // Rent sysvar account
                self.token_program.to_account_info(), // Token program account
            ],
            signer // Signer seeds for the CPI call
        )?;

        Ok(())
    }
}
