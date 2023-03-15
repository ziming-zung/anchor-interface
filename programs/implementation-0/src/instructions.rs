use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub account: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Data {
    pub data: u64,
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub account: Account<'info, Data>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub endpoint: Signer<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetDataParams {
    pub data: u64,
    pub test: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Default)]
pub struct ReturnData {
    pub data: String,
}