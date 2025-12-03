//-------------------------------------------------------------------------------
///
/// TASK: Implement the toggle lock functionality for the on-chain vault
/// 
/// Requirements:
/// - Toggle the locked state of the vault (locked becomes unlocked, unlocked becomes locked)
/// - Only the vault authority should be able to toggle the lock
/// - Emit a toggle lock event after successful state change
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::events::ToggleLockEvent;

#[derive(Accounts)]
pub struct ToggleLock<'info> {
    // TODO: Add required accounts and constraints
    pub vault_authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_authority.key().as_ref()],
        bump,
    )]
    pub vault : Account<'info, Vault>,
}

pub fn _toggle_lock(ctx: Context<ToggleLock>) -> Result<()> {
    // TODO: Implement toggle lock functionality
    let vault_account =  &mut ctx.accounts.vault;
    if vault_account.locked {
        vault_account.locked = false
    } else{ vault_account.locked = true}
    
    emit!(ToggleLockEvent{
        vault: vault_account.key(),
        vault_authority: ctx.accounts.vault_authority.key(),
        locked: vault_account.locked,
    });

    Ok(())
}