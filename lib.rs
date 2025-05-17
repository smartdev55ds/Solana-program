use anchor_lang::prelude::*;

declare_id!("YOUR_PROGRAM_ID_HERE");

#[program]
pub mod escrow {
    use super::*;

    // User funds go from PDA -> Escrow Vault PDA
    pub fn forward_to_escrow(ctx: Context<ForwardToEscrow>, amount: u64) -> Result<()> {
        let pda_seeds = &[
            b"user_pda",
            ctx.accounts.user.key.as_ref(),
            &[ctx.accounts.user_pda.bump],
        ];
        let signer = &[&pda_seeds[..]];

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user_pda.key(),
            &ctx.accounts.escrow_vault.key(),
            amount,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.user_pda.to_account_info(),
                ctx.accounts.escrow_vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            signer,
        )?;

        Ok(())
    }

    // Admin sends SOL from Escrow Vault PDA to user's wallet
    pub fn withdraw_to_user(ctx: Context<WithdrawToUser>, amount: u64) -> Result<()> {
        let vault_seeds = &[b"escrow_vault", &[ctx.accounts.escrow_vault.bump]];
        let signer = &[&vault_seeds[..]];

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.escrow_vault.key(),
            &ctx.accounts.user.key(),
            amount,
        );

        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.escrow_vault.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            signer,
        )?;

        Ok(())
    }
}


#[derive(Accounts)]
pub struct ForwardToEscrow<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: PDA for the user
    #[account(
        mut,
        seeds = [b"user_pda", user.key().as_ref()],
        bump
    )]
    pub user_pda: AccountInfo<'info>,

    /// CHECK: Central vault
    #[account(
        mut,
        seeds = [b"escrow_vault"],
        bump
    )]
    pub escrow_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawToUser<'info> {
    /// CHECK: This is the destination user wallet
    #[account(mut)]
    pub user: AccountInfo<'info>,

    /// CHECK: Escrow vault controlled by program
    #[account(
        mut,
        seeds = [b"escrow_vault"],
        bump
    )]
    pub escrow_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}