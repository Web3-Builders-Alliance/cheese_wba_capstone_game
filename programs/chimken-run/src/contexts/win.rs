use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::token_interface::TokenInterface;

use crate::states::Competition;

#[derive(Accounts)]
pub struct Winner<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"chimkenadmin", competition.admin.key().as_ref()],
        bump = competition.bump,
    )]
    competition: Account<'info, Competition>,
    #[account(
        mut,
        seeds = [b"treasury", competition.key().as_ref()],
        bump = competition.treasury_bump,
    )]
    treasury: SystemAccount<'info>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> Winner<'info> {
    pub fn winner(&mut self) -> Result<()> {
        msg!("Winner function start");

        self.competition
            .participants
            .sort_by(|a, b| b.score.cmp(&a.score));

        let winner = self.competition.participants[0].user;

        if self.user.key() == winner {
            msg!("User is the winner");

            let signer_seeds: [&[&[u8]]; 1] = [&[
                b"treasury",
                &self.competition.to_account_info().key.as_ref()[..],
                &[self.competition.bump],
            ]];

            let winner_account = Transfer {
                from: self.treasury.to_account_info(),
                to: self.user.to_account_info(),
            };

            let cpi_ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                winner_account,
                &signer_seeds,
            );

            msg!(
                "Transfering funds to winner {:?}",
                self.treasury.get_lamports()
            );
            transfer(cpi_ctx, self.treasury.get_lamports())?;
        }

        Ok(())
    }
}
