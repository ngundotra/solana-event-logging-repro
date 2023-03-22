use anchor_lang::prelude::*;

use program_c::{
    self,
    cpi::{accounts::Noop as NoopC, noop as noop_c},
    program::ProgramC,
};

declare_id!("4oXvagDQG11aajdFticQwyBCCoWnqpiEaVSPXKK1igsT");

#[program]
pub mod program_b {
    use super::*;

    pub fn noop(ctx: Context<Noop>, _args: String) -> Result<()> {
        let switch = ctx.accounts.switch.try_borrow_data()?[8];
        msg!("switch: {}", switch);
        if switch == 1 {
            noop_c(
                CpiContext::<NoopC>::new(
                    ctx.accounts.program_c.to_account_info(),
                    NoopC {
                        switch: ctx.accounts.switch.to_account_info(),
                    },
                ),
                "radio".to_string(),
            )?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Noop<'info> {
    /// CHECK: nothing lol idc
    pub switch: AccountInfo<'info>,
    pub program_c: Program<'info, ProgramC>,
}
