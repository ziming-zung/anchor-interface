// #region core
use anchor_lang::prelude::*;
pub mod primitives;

declare_id!("65t8LH4Yp8ugymLYGYAeuQRJNJoqshkByEWJJktgpoUa");

#[program]
mod invocation {
    use super::*;
    use anchor_lang::error::ErrorCode;
    use anchor_lang::solana_program::log::sol_log;
    use empty_implementation::cpi::accounts::SetData;
    use empty_implementation::SetDataParams;

    pub fn address_lookup(ctx: Context<MyAddresses>) -> anchor_lang::Result<()> {
        sol_log("address_lookup..");
        sol_log(format!("accounts: {:?}", ctx.accounts).as_str());
        sol_log(format!("remaining: {:?}", ctx.remaining_accounts).as_str());
        Ok(())
    }

    pub fn invoke(ctx: Context<PullStrings>) -> anchor_lang::Result<()> {
        sol_log("invoke..");

        let data = SetDataParams {
            data: 111u64,
            test: "xxx".to_string(),
        };

        {
            let mut acc_iter = ctx.remaining_accounts.iter();

            let cpi_program = acc_iter
                .next()
                .ok_or_else(|| ErrorCode::AccountNotEnoughKeys)?
                .to_account_info();
            let accounts = SetData {
                account: acc_iter
                    .next()
                    .ok_or_else(|| ErrorCode::AccountNotEnoughKeys)?
                    .to_account_info(),
                payer: acc_iter
                    .next()
                    .ok_or_else(|| ErrorCode::AccountNotEnoughKeys)?
                    .to_account_info(),
                endpoint: acc_iter
                    .next()
                    .ok_or_else(|| ErrorCode::AccountNotEnoughKeys)?
                    .to_account_info(),
            };
            msg!("shshshs");
            let cpi_ctx = CpiContext::new(cpi_program, accounts);

            // cpi call
            let result = empty_implementation::cpi::set_data(
                cpi_ctx.with_signer(&[&[b"seed", &[253]]]),
                data,
            )?;
            msg!("result:{:?}", result.get());
        }

        // https://github.com/coral-xyz/anchor/blob/37cc99c2b6c24e2bf03ff9d58ed451deb586ffef/lang/syn/src/codegen/program/common.rs#L7
        // let discriminator = {
        //     // how to gen discriminator
        //     let mut sighash = [0u8; 8];
        //     let preimage = b"global:set_data";
        //     sighash.copy_from_slice(
        //         &anchor_lang::solana_program::hash::hash(preimage).to_bytes()[..8],
        //     );
        //     // the value of sighash is discriminator
        //     sol_log(format!("sighash:{:?}", sighash).as_str());
        //     sighash.to_vec()
        // };

        // // let mut ix_data = <SetDataParams as anchor_lang::Discriminator>::discriminator().to_vec();
        // let mut ix_data = discriminator;
        // ix_data.append(&mut data);
        // let instruction = anchor_lang::solana_program::instruction::Instruction {
        //     program_id: ctx.accounts.implementation_id.key(),
        //     accounts,
        //     data: ix_data,
        // };

        // anchor_lang::solana_program::program::invoke_signed(
        //     &instruction,
        //     ctx.remaining_accounts,
        //     &[&[b"seed", &[253]]],
        // )?;
        // if let Some((actual_program_id, data)) = get_return_data() {
        //     sol_log(format!("actual_program_id:{}", actual_program_id).as_str());
        //     let mut data = data.as_slice();
        //     let return_data = primitives::ReturnData::deserialize(&mut data)?;
        //     sol_log(format!("get_return_data:{:?}", return_data).as_str());
        // }
        Ok(())
    }
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

// pub(crate) fn to_account_metadata(accounts: &[AccountInfo]) -> Vec<AccountMeta> {
//     accounts
//         .iter()
//         .map(|account| AccountMeta {
//             pubkey: *account.key,
//             is_signer: account.is_signer,
//             is_writable: account.is_writable,
//         })
//         .collect()
// }

#[derive(Accounts, Debug)]
pub struct PullStrings<'info> {
    pub signer: Signer<'info>,

    pub payer: Signer<'info>,

    /// CHECK:
    pub cpi_signer: UncheckedAccount<'info>,
}

#[derive(Accounts, Debug)]
pub struct MyAddresses<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK
    pub account_0: AccountInfo<'info>,
    /// CHECK
    pub account_1: AccountInfo<'info>,
}
