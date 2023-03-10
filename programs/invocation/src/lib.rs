// #region core
use anchor_lang::prelude::*;
// pub mod interface_instuction;
declare_id!("65t8LH4Yp8ugymLYGYAeuQRJNJoqshkByEWJJktgpoUa");

#[program]
mod invocation {

    use super::*;
    pub fn invoke(ctx: Context<PullStrings>) -> anchor_lang::Result<()> {
        anchor_lang::solana_program::log::sol_log("invoke..");

        let data: u64 = 111;
        let mut data = SetDataParams { data }.try_to_vec()?;
        let mut accounts = Vec::with_capacity(1);
        accounts.push(AccountMeta::new(
            ctx.accounts.signer.to_account_info().key(),
            false,
        ));
        let mut ix_data = <SetDataParams as anchor_lang::Discriminator>::discriminator().to_vec();
        ix_data.append(&mut data);
        let instruction = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.program_id.key(),
            accounts,
            data: ix_data,
        };
        anchor_lang::solana_program::program::invoke_unchecked(
            &instruction,
            &[ctx.accounts.signer.to_account_info()],
        )
        .map_err(|e| e.into())
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: just a program_id
    pub program_id: UncheckedAccount<'info>,
}

// #[derive(Clone)]
// pub struct GenericInterface;

// impl anchor_lang::CheckId for GenericInterface {
//     /// Allows any program to invoke
//     fn check_id(_id: &Pubkey) -> Result<()> {
//         sol_log("check program_id");
//         Ok(())
//     }
// }

#[derive(borsh::BorshSerialize)]
pub struct SetDataParams {
    pub data: u64,
}
impl anchor_lang::Discriminator for SetDataParams {
    const DISCRIMINATOR: [u8; 8] = [223, 114, 91, 136, 197, 78, 153, 153];
}
