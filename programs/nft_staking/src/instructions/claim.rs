use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::{mpl_token_metadata::instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts, ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata, MetadataAccount}, token::{approve, mint_to, revoke, Approve, Mint, MintTo, Revoke, Token, TokenAccount}};

use crate::{errors::StakeError, state::{StakeAccount, StakeConfig, UserAccount}};

#[derive(Accounts)]
pub struct Claim<'info>{

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
    )]
    pub rewards_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump = config.rewards_bump,
    )]
    pub reward_mint: Account<'info, Mint>,

    
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl <'info> Claim <'info> {

    pub fn claim(&mut self)-> Result<()>{

        let cpi_program = self.token_program.to_account_info();

        let seeds = &[
            b"config".as_ref(),
            &[self.config.bump]
        ];     
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = MintTo{
            mint: self.reward_mint.to_account_info(),
            to: self.rewards_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, self.user_account.points as u64 * 10_u64.pow(self.reward_mint.decimals as u32))?;

        self.user_account.points = 0;


        Ok(())
    }
}