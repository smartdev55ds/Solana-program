#[account]
pub struct EscrowAccount {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub status: EscrowStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EscrowStatus {
    Pending,
    Released,
    Refunded,
}
