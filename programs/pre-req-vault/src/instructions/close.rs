// ==========================================================
// FILE: instructions/close.rs - file 7
// PURPOSE: Drains all remaining SOL from the Vault back to the user, 
// and completely deletes the VaultState account from the blockchain, 
// refunding its rent (SOL) back to the user.
// ==========================================================

use crate::state::VaultState;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // The Vault PDA. `mut` because its SOL balance will drop to 0.
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    // The state account.
    // `mut` because we are deleting it.
    // `close = user` is Anchor magic. After this instruction finishes, 
    // Anchor will delete this account and send its rent (SOL) to the `user`.
    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user,
    )]
    pub vault_state: Account<'info, VaultState>,

    system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        // Just like withdraw, the Vault is sending the SOL, so the Vault PDA must sign.
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(System::id(), cpi_accounts, signer_seeds);

        // Notice we don't pass an `amount` argument. 
        // We use `.lamports()` to grab the ENTIRE balance of the vault 
        // and send all of it to the user.
        transfer(cpi_ctx, self.vault.lamports())?;

        Ok(())
    }
}