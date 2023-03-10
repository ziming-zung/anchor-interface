use anchor_lang::prelude::*;

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
}

#[account]
pub struct Data {
    pub data: u64,
}
