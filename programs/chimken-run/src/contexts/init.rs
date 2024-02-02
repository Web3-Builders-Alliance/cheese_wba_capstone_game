use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenInterface;

use crate::states::{Competition, UserEntry};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        space = Competition::INIT_SPACE,
        payer = admin,
        seeds = [b"chimkenadmin", admin.key.as_ref()],
        bump
    )]
    competition: Account<'info, Competition>,
    #[account(
        seeds = [b"treasury", competition.key().as_ref()],
        bump,
    )]
    treasury: SystemAccount<'info>,
    #[account(
        seeds = [b"fee", competition.key().as_ref()],
        bump,
    )]
    fee: SystemAccount<'info>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, entry_price: u64, duration: u64, bumps: &InitializeBumps) -> Result<()> {
        let user_entries: Vec<UserEntry> = Vec::new();
        // TODO:: checker for entry price if less than zero
        self.competition.set_inner(Competition {
            admin: self.admin.key(),
            treasury_bump: bumps.treasury,
            fee_bump: bumps.fee,
            bump: bumps.competition,
            entry_price,
            duration,
            participants: user_entries,
        });

        Ok(())
    }
}
