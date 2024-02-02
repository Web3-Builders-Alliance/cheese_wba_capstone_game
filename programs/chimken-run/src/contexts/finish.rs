use anchor_lang::prelude::*;

use crate::states::Competition;

#[derive(Accounts)]
pub struct Finish<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"chimkenadmin", competition.admin.key().as_ref()],
        bump = competition.bump,
    )]
    competition: Account<'info, Competition>,
    system_program: Program<'info, System>,
}

impl<'info> Finish<'info> {
    pub fn finish(&mut self, score: u32) -> Result<()> {
        msg!("Finish function start");

        if let Some(user_entry) = self
            .competition
            .participants
            .iter_mut()
            .find(|x| x.user == self.user.key())
        {
            user_entry.score = score;
            user_entry.game_count += 1;
        }

        Ok(())
    }
}
