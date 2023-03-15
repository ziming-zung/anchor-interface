use anchor_lang::prelude::*;
pub mod instructions;

pub use instructions::*;
declare_id!("7oMeM6mg429aeE9kJtskWNQmQU53tAFCtPzwQARUTF2h");

#[program]
pub mod implementation_1 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<ReturnData> {
        let account = &mut ctx.accounts.account;
        anchor_lang::solana_program::log::sol_log("implementation_1");
        account.data = data;
        let return_data = ReturnData {
            data: "result from implementation_1".to_string(),
        };
        Ok(return_data)
    }
}
