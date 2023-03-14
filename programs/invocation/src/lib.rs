// #region core
use anchor_lang::prelude::*;
pub mod interface_instuction;
pub mod primitives;

declare_id!("65t8LH4Yp8ugymLYGYAeuQRJNJoqshkByEWJJktgpoUa");

#[program]
mod invocation {

    use super::*;
    use anchor_lang::solana_program::{log::sol_log, program::get_return_data, pubkey};

    pub fn address_lookup(ctx: Context<MyAddresses>) -> anchor_lang::Result<()> {
        sol_log("address_lookup..");
        sol_log(format!("accounts: {:?}", ctx.accounts).as_str());
        sol_log(format!("remaining: {:?}", ctx.remaining_accounts).as_str());
        Ok(())
    }

    pub fn invoke(ctx: Context<PullStrings>) -> anchor_lang::Result<()> {
        sol_log("invoke..");

        let data: u64 = 111;
        let mut data = SetDataParams { data }.try_to_vec()?;
        // let mut accounts = Vec::with_capacity(3);
        // sol_log(format!("remaining: {:?}", ctx.remaining_accounts).as_str());
        ctx.remaining_accounts
            .iter()
            .enumerate()
            .for_each(|(index, v)| {
                msg!(format!("remaining account index:{:?}, {:?}", index, v.key).as_str());
            });
        
        let mut accounts = to_account_metadata(ctx.remaining_accounts);

        let mut account_infos = ctx.remaining_accounts.to_vec();
        account_infos.get_mut(2).and_then(|info| {
            msg!(format!("endpoint: {:?}", info.key).as_str());
            info.is_signer = true;        
            Some(info)
        });

        let account_infos = account_infos.as_slice();

        // accounts.push(AccountMeta {
        //     pubkey: ctx.remaining_accounts[0].key(),
        //     is_signer: false,
        //     is_writable: false,
        // });
        // accounts.push(AccountMeta {
        //     pubkey: ctx.remaining_accounts[1].key(),
        //     is_signer: false,
        //     is_writable: false,
        // });

        // let mut account_infos = Vec::with_capacity(3);
        // account_infos.push(ctx.accounts.signer.clone().to_account_info());
        // account_infos.push(ctx.remaining_accounts[0].clone());
        // account_infos.push(ctx.remaining_accounts[1].clone());

        // accounts.push(AccountMeta::new(ctx.remaining_accounts[0].key(), true));

        let discriminator = {
            // how to gen discriminator
            let mut sighash = [0u8; 8];
            let preimage = b"global:set_data";
            sighash.copy_from_slice(
                &anchor_lang::solana_program::hash::hash(preimage).to_bytes()[..8],
            );
            // the value of sighash is discriminator
            sol_log(format!("sighash:{:?}", sighash).as_str());
            sighash.to_vec()
        };
        // let mut ix_data = <SetDataParams as anchor_lang::Discriminator>::discriminator().to_vec();
        let mut ix_data = discriminator;
        ix_data.append(&mut data);
        let instruction = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.program_id.key(),
            accounts,
            data: ix_data,
        };
        msg!("hahahah----11111111");
        if let Ok(pk) =
            Pubkey::create_program_address(&[b"seed", &[253]], ctx.accounts.program_id.key)
        {
            msg!(format!("pk:{:?}", pk).as_str());
        }
        
        anchor_lang::solana_program::program::invoke_signed(
            &instruction,
            account_infos,
            &[&[b"seed", &[253]]],            
        )?;

        msg!("hahahah");
        // if let Some((actual_program_id, data)) = get_return_data() {
        //     sol_log(format!("actual_program_id:{}", actual_program_id).as_str());
        //     let mut data = data.as_slice();
        //     let return_data = primitives::ReturnData::deserialize(&mut data)?;
        //     sol_log(format!("get_return_data:{:?}", return_data).as_str());
        // }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub payer: Signer<'info>,

    /// CHECK: just a program_id
    pub program_id: UncheckedAccount<'info>,
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

// TODO hash by 2 string
impl anchor_lang::Discriminator for SetDataParams {
    const DISCRIMINATOR: [u8; 8] = [223, 114, 91, 136, 197, 78, 153, 153];
}

pub(crate) fn to_account_metadata(accounts: &[AccountInfo]) -> Vec<AccountMeta> {
    accounts
        .iter()
        .map(|account| AccountMeta {
            pubkey: *account.key,
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect()
}
