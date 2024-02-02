use anchor_lang::prelude::*;

#[account]
pub struct Competition {
    pub admin: Pubkey,
    pub treasury_bump: u8,
    pub fee_bump: u8,
    pub bump: u8,
    pub entry_price: u64,
    pub duration: u64,
    pub participants: Vec<UserEntry>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UserEntry {
    pub score: u32,
    pub game_count: u8,
    pub user: Pubkey,
    pub nft_mint: Pubkey,
    pub is_paid: bool,
}

impl Space for Competition {
    const INIT_SPACE: usize = 8 + 32 + 1 + 1 + 1 + 8 + 8 + (70 + 4);
}
