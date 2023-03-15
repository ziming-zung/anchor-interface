use anchor_lang::prelude::*;
pub mod instructions;

pub use instructions::*;
declare_id!("6JLEcYUCKbfo1Gn6TUoMaE7CoeiXK1JyjyavLeSaCC4y");

#[program]
pub mod empty_implementation {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("empty: initialize");
        Ok(())
    }

    pub fn set_data(_ctx: Context<SetData>, _params: SetDataParams) -> Result<ReturnData> {
        msg!("empty: set_data");
        Ok(ReturnData::default())
    }
}
