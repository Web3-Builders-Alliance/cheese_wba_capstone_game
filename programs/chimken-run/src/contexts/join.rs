use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::token_interface::TokenInterface;

use crate::states::{Competition, UserEntry};

#[derive(Accounts)]
pub struct Join<'info> {
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

impl<'info> Join<'info> {
    pub fn join(&mut self) -> Result<()> {
        msg!("Join function start");

        match self
            .competition
            .participants
            .iter()
            .find(|x| x.user == self.user.key() && x.nft_mint == self.user.key())
        {
            Some(_) => {
                msg!("User already joined competition");
                Ok(())
            }
            None => {
                self.competition.participants.push(UserEntry {
                    score: 0,
                    game_count: 0,
                    user: self.user.key(),
                    nft_mint: self.user.key(),
                    is_paid: true,
                });

                msg!("New user joined competition");
                Ok(())
            }
        }
    }

    pub fn realloc(&mut self) -> Result<()> {
        msg!("Realloc function start");
        let new_size = self.competition.to_account_info().data_len() + 70;
        let lamports_required = (Rent::get()?).minimum_balance(new_size);
        let addtnl_rent = lamports_required - self.competition.to_account_info().lamports();

        let addtnl_rent_acct = Transfer {
            from: self.user.to_account_info(),
            to: self.competition.to_account_info().clone(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), addtnl_rent_acct);

        transfer(cpi_ctx, addtnl_rent)?;
        self.competition
            .to_account_info()
            .realloc(new_size, false)?;

        msg!("Competition reallocated, new rent price {:?}", addtnl_rent);

        Ok(())
    }

    pub fn pay(&mut self, entry_price: u64) -> Result<()> {
        msg!("Pay function start");
        msg!("Competition reallocated, new rent price {:?}", entry_price);
        msg!(
            "Competition reallocated, new rent price {:?}",
            self.competition.entry_price
        );

        if self.competition.entry_price != entry_price {
            msg!("User paid wrong entry fee");
        } else {
            let join_acct = Transfer {
                from: self.user.to_account_info(),
                to: self.treasury.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), join_acct);

            msg!("User paid entry fee");
            transfer(cpi_ctx, entry_price)?
        };

        Ok(())
    }
}
