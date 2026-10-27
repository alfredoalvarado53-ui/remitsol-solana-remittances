use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("RemitSoL11111111111111111111111111111111111");

#[program]
pub mod remitsol {
    use super::*;

    /// Initialize a new remittance escrow stream
    pub fn create_remittance(
        ctx: Context<CreateRemittance>,
        amount: u64,
        stream_duration_seconds: i64,
    ) -> Result<()> {
        let remittance = &mut ctx.accounts.remittance;
        let clock = Clock::get()?;

        remittance.sender = ctx.accounts.sender.key();
        remittance.recipient = ctx.accounts.recipient.key();
        remittance.mint = ctx.accounts.mint.key();
        remittance.total_amount = amount;
        remittance.claimed_amount = 0;
        remittance.start_time = clock.unix_timestamp;
        remittance.end_time = clock.unix_timestamp + stream_duration_seconds;
        remittance.bump = ctx.bumps.remittance;

        // Transfer funds from sender to escrow token account
        let cpi_accounts = Transfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.escrow_vault.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

        msg!("RemitSol: Remittance stream initialized for {} tokens", amount);
        Ok(())
    }

    /// Recipient claims available streamed funds
    pub fn claim_stream(ctx: Context<ClaimStream>) -> Result<()> {
        let remittance = &mut ctx.accounts.remittance;
        let clock = Clock::get()?;

        // Calculate vested amount based on elapsed time
        let elapsed = clock.unix_timestamp.saturating_sub(remittance.start_time);
        let duration = remittance.end_time.saturating_sub(remittance.start_time).max(1);

        let vested_amount = if clock.unix_timestamp >= remittance.end_time {
            remittance.total_amount
        } else {
            (remittance.total_amount as u128 * elapsed as u128 / duration as u128) as u64
        };

        let claimable = vested_amount.saturating_sub(remittance.claimed_amount);
        require!(claimable > 0, RemitSolError::NoFundsAvailable);

        remittance.claimed_amount += claimable;

        // Transfer claimed funds to recipient
        let sender_key = remittance.sender;
        let seeds = &[
            b"remittance",
            sender_key.as_ref(),
            &[remittance.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_vault.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: remittance.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new_with_signer(cpi_program, cpi_accounts, signer), claimable)?;

        msg!("RemitSol: Recipient claimed {} tokens", claimable);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateRemittance<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: Recipient pubkey to receive stream
    pub recipient: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = sender,
        space = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 1,
        seeds = [b"remittance", sender.key().as_ref()],
        bump
    )]
    pub remittance: Account<'info, RemittanceEscrow>,
    #[account(
        mut,
        constraint = sender_token_account.mint == mint.key(),
        constraint = sender_token_account.owner == sender.key()
    )]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = sender,
        token::mint = mint,
        token::authority = remittance,
        seeds = [b"vault", remittance.key().as_ref()],
        bump
    )]
    pub escrow_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ClaimStream<'info> {
    #[account(mut)]
    pub recipient: Signer<'info>,
    #[account(
        mut,
        seeds = [b"remittance", remittance.sender.as_ref()],
        bump = remittance.bump,
        constraint = remittance.recipient == recipient.key()
    )]
    pub remittance: Account<'info, RemittanceEscrow>,
    #[account(
        mut,
        seeds = [b"vault", remittance.key().as_ref()],
        bump
    )]
    pub escrow_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = recipient_token_account.mint == remittance.mint,
        constraint = recipient_token_account.owner == recipient.key()
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct RemittanceEscrow {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub mint: Pubkey,
    pub total_amount: u64,
    pub claimed_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub bump: u8,
}

#[error_code]
pub enum RemitSolError {
    #[msg("No funds are currently vested or available to claim.")]
    NoFundsAvailable,
}
