use anchor_lang::prelude::*;

use program_b::{
    self,
    cpi::{accounts::Noop as NoopB, noop as noop_b},
    program::ProgramB,
};
use program_c::{
    self,
    cpi::{accounts::Noop as NoopC, noop as noop_c},
    program::ProgramC,
};

declare_id!("C2V33PChTC4TMVQMNS9NGKD3HVxhqoUSLbX2MtAaih2R");

#[program]
pub mod eventonese {
    use super::*;

    pub fn joke(ctx: Context<Initialize>, switch: u8) -> Result<()> {
        msg!("What's the best joke in the world?");

        for _ in 0..1000 {
            msg!("0123456789");
        }

        if switch == 0 {
            noop_b(
                CpiContext::<NoopB>::new(
                    ctx.accounts.program_b.to_account_info(),
                    NoopB {
                        switch: ctx.accounts.switch.to_account_info(),
                        program_c: ctx.accounts.program_c.to_account_info(),
                    },
                ),
                "no soap".to_string(),
            )?;

            noop_c(
                CpiContext::<NoopC>::new(
                    ctx.accounts.program_c.to_account_info(),
                    NoopC {
                        switch: ctx.accounts.switch.to_account_info(),
                    },
                ),
                "radio".to_string(),
            )?;
        } else {
            let switch_account = ctx.accounts.switch.to_account_info();
            {
                let mut data = switch_account.try_borrow_mut_data()?;
                data[8] = 1;
            }

            noop_b(
                CpiContext::<NoopB>::new(
                    ctx.accounts.program_b.to_account_info(),
                    NoopB {
                        switch: ctx.accounts.switch.to_account_info(),
                        program_c: ctx.accounts.program_c.to_account_info(),
                    },
                ),
                "no soap".to_string(),
            )?;
            {
                let mut data = switch_account.try_borrow_mut_data()?;
                data[8] = 0;
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        seeds=[b"eventonese".as_ref()],
        bump,
        space = 8 + 1,
        payer=payer
    )]
    pub switch: Account<'info, Switch>,
    pub program_b: Program<'info, ProgramB>,
    pub program_c: Program<'info, ProgramC>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Switch {
    pub switch: u8,
}
