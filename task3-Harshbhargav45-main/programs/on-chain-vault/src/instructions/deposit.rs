//-------------------------------------------------------------------------------
///
/// TASK: Implement the deposit functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the user has enough balance to deposit
/// - Verify that the vault is not locked
/// - Transfer lamports from user to vault using CPI (Cross-Program Invocation)
/// - Emit a deposit event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
// use anchor_lang::solana_program::system_instruction::transfer;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::DepositEvent;

#[derive(Accounts)]
pub struct Deposit<'info> {
    // TODO: Add required accounts and constraints
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut
    )]
    pub vault : Account<'info, Vault>,
    pub system_program : Program<'info, System>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // TODO: Implement deposit functionality
    let vault_account = &mut ctx.accounts.vault;

    vault_account.vault_authority = ctx.accounts.user.key();

    if vault_account.locked {
        return Err(error!(VaultError::VaultLocked));
    }
    if ctx.accounts.user.to_account_info().get_lamports() < amount{
        return Err(error!(VaultError::InsufficientBalance))
    }
    let transfer_accounts = Transfer{
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.vault.to_account_info()
    };
    let cpi_context = CpiContext::new(ctx.accounts.system_program.to_account_info(), transfer_accounts);
    transfer(cpi_context, amount)?;

    emit!(DepositEvent{
        amount,
        user: ctx.accounts.user.key(),
        vault: ctx.accounts.vault.key(),
    });

    Ok(())

}
