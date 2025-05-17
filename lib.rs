use anchor_lang::prelude::*;

declare_id!("YOUR_PROGRAM_ID_HERE");

#[program]
pub mod escrow {
    use super::*;

    // Move funds from user PDA to Escrow Vault PDA
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
}

#[derive(Accounts)]
pub struct ForwardToEscrow<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: User-specific PDA
    #[account(
        mut,
        seeds = [b"user_pda", user.key().as_ref()],
        bump
    )]
    pub user_pda: AccountInfo<'info>,

    /// CHECK: Escrow vault PDA
    #[account(
        mut,
        seeds = [b"escrow_vault"],
        bump
    )]
    pub escrow_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
