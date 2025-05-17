use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("YourProgramIdGoesHere");

#[program]
pub mod escrow_system {
    use super::*;

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        amount: u64,
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.buyer = *ctx.accounts.buyer.key;
        escrow.seller = *ctx.accounts.seller.key;
        escrow.token_mint = ctx.accounts.token_mint.key();
        escrow.amount = amount;
        escrow.status = EscrowStatus::Pending;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;
        require_eq!(escrow.status, EscrowStatus::Pending, EscrowError::InvalidState);

        let cpi_accounts = Transfer {
            from: ctx.accounts.buyer_token_account.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, escrow.amount)?;

        Ok(())
    }

    pub fn release(ctx: Context<AdminAction>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        require_eq!(escrow.status, EscrowStatus::Pending, EscrowError::InvalidState);

        let escrow_seeds = &[
            b"escrow".as_ref(),
            escrow.buyer.as_ref(),
            escrow.seller.as_ref(),
            &[ctx.bumps.escrow_token_account],
        ];

        let signer_seeds = &[&escrow_seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_token_account.to_account_info(),
            to: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.escrow_token_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        token::transfer(cpi_ctx, escrow.amount)?;

        escrow.status = EscrowStatus::Released;
        Ok(())
    }

    pub fn refund(ctx: Context<AdminAction>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        require_eq!(escrow.status, EscrowStatus::Pending, EscrowError::InvalidState);

        let escrow_seeds = &[
            b"escrow".as_ref(),
            escrow.buyer.as_ref(),
            escrow.seller.as_ref(),
            &[ctx.bumps.escrow_token_account],
        ];
        let signer_seeds = &[&escrow_seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_token_account.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.escrow_token_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        token::transfer(cpi_ctx, escrow.amount)?;

        escrow.status = EscrowStatus::Refunded;
        Ok(())
    }
}
