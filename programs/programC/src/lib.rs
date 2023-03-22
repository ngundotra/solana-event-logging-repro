use anchor_lang::prelude::*;

declare_id!("CiKq5LvLJYpGsntqvPMmKefPTJtJ2UcF97pECXchvhTD");

#[program]
pub mod program_c {
    use super::*;

    pub fn noop(_ctx: Context<Noop>, _args: String) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Noop<'info> {
    /// CHECK: this is actually an unnecessary account
    pub switch: AccountInfo<'info>,
}
