use anchor_lang::prelude::*;

declare_id!("5wHDR76LjVN75NwwHT4L85vDvzCDcyo64YZ5hkaAr2X1");

#[program]
pub mod flipper {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let switchAccount = &mut ctx.accounts.switchAccount;
        switchAccount.state = true;
        Ok(())
    }

    pub fn flip(ctx: Context<Flip>) -> Result<()> {
        let switchAccount = &mut ctx.accounts.switchAccount;
        if (switchAccount.state){
            switchAccount.state = false;
        }
        else {
            switchAccount.state = true;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub switchAccount: Account<'info, SwitchAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Flip<'info> {
    #[account(mut)]
    pub switchAccount: Account<'info, SwitchAccount>,
}

#[account]
pub struct SwitchAccount {
    pub state: bool,
}