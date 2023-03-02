use anchor_lang::prelude::*;
use anchor_spl::token::{Token};
declare_id!("8EospLLFmzvnwL3BLpRL9kCA3sFEkBNJQTaURrcJfQ3k");

    const SEED_DATA:&[u8] = b"ABC";
#[program]
pub mod sol_operation {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("PDA IS Initialize");
        Ok(())
    }
    pub fn sol_transfer(ctx: Context<SolTransfer>,amount:f64) ->Result<()>{
        let sender = ctx.accounts.sender.to_account_info();
        let reciver = ctx.accounts.reciver.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();
        let instruction = anchor_lang::system_program::Transfer{
            from:sender.to_account_info(),
            to: reciver.to_account_info(),
        };

        let temp = amount*1000_000_000.0;
        let lamports= temp as u64;
        anchor_lang::system_program::transfer(
            CpiContext::new(system_program, instruction)
            , lamports)?;
        Ok(())
    }

    pub fn transfer_sol_pda(ctx: Context<TransferPDA>,amount:f64)-> Result<()>{
        let owner = ctx.accounts.owner.to_account_info();
        let user_pda = ctx.accounts.user_pda.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();
        
        let instruction = anchor_lang::system_program::Transfer{
            from:owner,
            to:user_pda
        };

        let temp = amount*1000_000_000.0;
        let lamports= temp as u64;
        anchor_lang::system_program::transfer(
            CpiContext::new(system_program, instruction), lamports)?;
        Ok(())
    }
    pub fn pdato_pda(ctx: Context<PDAtoPDA>,amount:f64)-> Result<()>{
        let owner = ctx.accounts.owner.to_account_info();
        let userpda = &mut ctx.accounts.authority_pda; 
        let userata = ctx.accounts.sender_ata.to_account_info();
        let receiverata = ctx.accounts.reciver_ata.to_account_info();
        let token_program = ctx.accounts.token_program.to_account_info();

        let (main,bump) = Pubkey::find_program_address(
            &[SEED_DATA.as_ref(),owner.key().as_ref()], ctx.program_id);
        let instruction = anchor_spl::token::Transfer{
            authority:userpda.to_account_info(),
            from:userata.to_account_info(),
            to:receiverata.to_account_info(),
        };
        let temp = amount*1000_000_000.0 as f64;
        let lamports = temp as u64;
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(token_program, instruction, &[&[b"ABC",owner.key().as_ref(),&[bump]]]), lamports)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    ///CHECK:
    #[account(mut)]
    pub owner: Signer<'info>,

    ///CHECK:
    #[account(mut)]
    pub user: AccountInfo<'info>,

    ///CHECK:
    #[account(
        init,
        payer=user,
        seeds=[b"ABC".as_ref(),user.key().as_ref()],
        bump,
        space=8
    )]
    pub user_info: AccountInfo<'info>,

    pub system_program: Program<'info,System>,
}
#[derive(Accounts)]
pub struct TransferPDA<'info>{
    pub owner: Signer<'info>,

    ///CHECK:
    #[account(
        mut,
        seeds=[b"ABC".as_ref(),owner.key().as_ref()],
        bump,
    )]
    pub user_pda: AccountInfo<'info>,

    pub system_program: Program<'info,System>,
}
#[derive(Accounts)]
pub struct SolTransfer<'info>{
    pub owner: Signer<'info>,

    ///CHECK:
    #[account(mut)]
    pub sender: AccountInfo<'info>,

    ///CHECK:
    #[account(mut)]
    pub reciver: AccountInfo<'info>,

    pub system_program: Program<'info,System>,
}
#[derive(Accounts)]
pub struct PDAtoPDA<'info>{
    ///CHECK:
    #[account(mut)]
    pub owner: AccountInfo<'info>,

    ///CHECK:
    #[account(
        mut,
        seeds=[b"ABC",owner.key().as_ref()],
        bump
    )]
    pub authority_pda: AccountInfo<'info>,
    
    ///CHECK:
    #[account(mut)]
    pub reciver_ata: AccountInfo<'info>,

    ///CHECK:
    #[account(mut)]
    pub sender_ata: AccountInfo<'info>,

    pub token_program: Program<'info,Token>,
}