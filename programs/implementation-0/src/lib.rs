use anchor_lang::prelude::*;
pub mod instructions;

pub use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod implementation_0 {

    use crate::*;
    use anchor_lang::solana_program::log::sol_log;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<ReturnData> {
        let account = &mut ctx.accounts.account;
        sol_log("implementation_0");
        msg!("signer:{:?}", ctx.accounts.endpoint);
        account.data = data;

        let return_data = ReturnData {
            data: "result from implementation_0".to_string(),
        };
        ctx.remaining_accounts.iter().for_each(|v| {
            sol_log(format!("remaining accounts:{:?}", v.key).as_str());
        });

        Ok(return_data)
    }
}
