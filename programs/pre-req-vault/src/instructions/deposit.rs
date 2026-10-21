// ==========================================================
// FILE: instructions/deposit.rs - file 5
// PURPOSE: Allows a user to deposit SOL into the Vault PDA. 
// It requires the user to sign the transaction and uses a CPI 
// to the System Program to move the funds.
// ==========================================================

// Imports the VaultState struct from our state.rs file.
use crate::state::VaultState;

// Imports Anchor's prelude and the specific System Program tools 
// needed to transfer SOL (transfer function and Transfer struct).
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

// Defines the accounts required for the Deposit instruction.
#[derive(Accounts)]
pub struct Deposit<'info> {
    // The user depositing the funds.
    // `mut` because their SOL balance will decrease.
    // `Signer` because they must authorize the transfer.
    #[account(mut)]
    pub user: Signer<'info>,

    // The Vault PDA holding the SOL.
    // `mut` because its SOL balance will increase.
    // `seeds` derives the address using "vault" + the vault_state's address.
    // `bump = vault_state.vault_bump` tells Anchor to use the saved bump 
    // from initialize.rs instead of recalculating it (saves compute units).
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    // The state account.
    // We only need to read this to get the `state_bump` and `vault_bump`. 
    // Notice we don't use `init` here; the account already exists.
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    // The System Program. Required whenever you create accounts or move SOL.
    system_program: Program<'info, System>,
}

// The actual logic for the deposit.
impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // 1. Set up the transfer parameters (from: user, to: vault).
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        // 2. Create the CPI context.
        // We use CpiContext::new (NOT new_with_signer) because the User 
        // is the one signing the transaction, not the Vault PDA. 
        // The system program verifies the user's signature.
        let cpi_ctx = CpiContext::new(System::id(), cpi_accounts);

        // 3. Execute the transfer.
        // This calls the Solana System Program to move the `amount` of lamports.
        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}