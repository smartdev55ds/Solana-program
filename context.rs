#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct InitializeEscrow<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: trusted by buyer
    pub seller: UncheckedAccount<'info>,
    pub token_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = buyer,
        seeds = [b"escrow", buyer.key().as_ref(), seller.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<EscrowAccount>(),
    )]
    pub escrow: Account<'info, EscrowAccount>,
    #[account(
        init,
        payer = buyer,
        token::mint = token_mint,
        token::authority = escrow_token_account,
        seeds = [b"escrow", buyer.key().as_ref(), seller.key().as_ref()],
        bump
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub escrow: Account<'info, EscrowAccount>,
}

#[derive(Accounts)]
pub struct AdminAction<'info> {
    #[account(mut, address = ADMIN_PUBKEY)] // Hardcoded admin key
    pub admin: Signer<'info>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub escrow: Account<'info, EscrowAccount>,
}
