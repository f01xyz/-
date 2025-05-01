use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use anchor_lang::solana_program::program_option::COption;

declare_id!("7dFbQVWEmYpKYRACeaED79wuMmsLxjMccf8hS9jLNy5i");

#[program]
pub mod florin_token {
    use super::*;

    /// Initialize the Florin token mint
    /// This is a one-time setup for the Florin token
    /// In the future, this will support Token-2022 extensions
    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        decimals: u8,
    ) -> Result<()> {
        msg!("Initializing Florin token mint");
        
        // The actual mint initialization is handled by the system program and token program
        // through the account constraints in the InitializeMint struct
        
        // In the future, we'll add Token-2022 extensions here:
        // - Permanent freeze capability
        // - Non-transferable option
        // - Confidential transfers
        
        Ok(())
    }

    /// Mint Florin tokens to a contributor account
    /// This is a permissioned instruction that can only be called by the mint authority
    pub fn mint_florin(
        ctx: Context<MintFlorin>,
        amount: u64,
        metadata: Option<ContributionMetadata>,
    ) -> Result<()> {
        msg!("Minting {} Florin tokens", amount);
        
        // Mint tokens to the recipient
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.recipient.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            amount,
        )?;
        
        // If metadata is provided, store it
        if let Some(metadata) = metadata {
            // In a real implementation, we would store this metadata on-chain
            // For now, we just log it
            msg!("Contribution metadata: {:?}", metadata);
        }
        
        Ok(())
    }

    /// Burn Florin tokens
    /// Used when contributors withdraw from the network or transfer between contexts
    pub fn burn_florin(
        ctx: Context<BurnFlorin>,
        amount: u64,
    ) -> Result<()> {
        msg!("Burning {} Florin tokens", amount);
        
        // Burn tokens from the owner's account
        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.from.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;
        
        Ok(())
    }
}

/// Accounts required for initializing the Florin token mint
#[derive(Accounts)]
pub struct InitializeMint<'info> {
    /// The mint account to initialize
    #[account(
        init,
        payer = payer,
        mint::decimals = 6, // Standard for SPL tokens
        mint::authority = mint_authority.key(),
    )]
    pub mint: Account<'info, Mint>,
    
    /// The authority that can mint new tokens
    /// This will initially be a single authority but can be upgraded to multisig or DAO later
    pub mint_authority: Signer<'info>,
    
    /// The account paying for the initialization
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// Required system program
    pub system_program: Program<'info, System>,
    
    /// Required token program
    /// In the future, this will be upgraded to use the Token-2022 program
    pub token_program: Program<'info, Token>,
    
    /// Required rent sysvar
    pub rent: Sysvar<'info, Rent>,
}

/// Accounts required for minting Florin tokens
#[derive(Accounts)]
pub struct MintFlorin<'info> {
    /// The mint account
    #[account(
        mut,
        constraint = mint.mint_authority == COption::Some(mint_authority.key()) @ ErrorCode::InvalidMintAuthority
    )]
    pub mint: Account<'info, Mint>,
    
    /// The mint authority
    pub mint_authority: Signer<'info>,
    
    /// The account to receive the minted tokens
    #[account(mut)]
    pub recipient: Account<'info, TokenAccount>,
    
    /// Required token program
    pub token_program: Program<'info, Token>,
}

/// Accounts required for burning Florin tokens
#[derive(Accounts)]
pub struct BurnFlorin<'info> {
    /// The mint account
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    /// The account to burn tokens from
    #[account(
        mut,
        constraint = from.owner == owner.key() @ ErrorCode::InvalidOwner
    )]
    pub from: Account<'info, TokenAccount>,
    
    /// The owner of the token account
    pub owner: Signer<'info>,
    
    /// Required token program
    pub token_program: Program<'info, Token>,
}

/// Metadata for a contribution
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ContributionMetadata {
    /// ID of the voucher that was redeemed for this contribution
    pub voucher_id: Option<String>,
    
    /// Reference to the contributor
    pub contributor_ref: Option<String>,
    
    /// Campaign or context for this contribution
    pub campaign: Option<String>,
    
    /// Additional notes
    pub notes: Option<String>,
}

/// Error codes for the Florin token program
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid mint authority")]
    InvalidMintAuthority,
    
    #[msg("Invalid owner")]
    InvalidOwner,
}
