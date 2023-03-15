use anchor_lang::prelude::*;
// use invocation::interface_instuction::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod implementation_0 {

    use anchor_lang::solana_program::log::sol_log;

    use super::*;

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

// TODO extract struct to file/other lib
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub account: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts, AnchorSerialize)]
pub struct SetData<'info> {
    #[account(mut)]
    pub account: Account<'info, Data>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub endpoint: Signer<'info>,
}

#[account]
pub struct Data {
    pub data: u64,
}

/// why can't import struct from other crate
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct ReturnData {
    pub data: String,
}
