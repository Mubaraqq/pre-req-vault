// ==========================================================
// FILE: instructions/initialize.rs - file 4
// PURPOSE: Sets up the Vault. It creates the VaultState account 
// (which stores our seeds/bumps) and derives the Vault PDA 
// (which will hold the SOL). We only do this once.
// ==========================================================

// Imports the VaultState struct we defined in state.rs.
use crate::state::VaultState;

// Imports Anchor's standard library.
use anchor_lang::prelude::*;

// The #[derive(Accounts)] macro tells Anchor: "These are the accounts 
// required for the Initialize instruction. Validate them before running the logic."
#[derive(Accounts)]
pub struct Initialize<'info> {
    // The user calling this. 
    // `mut` means we can change their balance (they pay for the account rent).
    // `Signer` means they must cryptographically sign the transaction.
    #[account(mut)]
    pub user: Signer<'info>,

    // The state account.
    // `init` tells Anchor to create this account. If it already exists, the transaction fails.
    // `payer = user` means the user pays the SOL rent.
    // `seeds` defines the PDA address: the word "state" + the user's public key.
    // `bump` tells Anchor to find the canonical bump automatically.
    // `space` allocates exactly the right size (8 bytes for Anchor discriminator + whatever the struct needs).
    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = 8 + VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,

    // The Vault account (where the SOL will live).
    // Notice there is NO `init` here. We aren't creating it yet. 
    // A SystemAccount (a basic wallet) doesn't need `init` to exist. 
    // We just derive its address using the seeds "vault" + the vault_state's address.
    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    // Required by Solana anytime you create an account (to pay rent).
    pub system_program: Program<'info, System>,
}

// This is the actual logic that runs after the accounts are validated.
impl<'info> Initialize<'info> {
    // Notice it takes `bumps: &InitializeBumps`. 
    // Anchor automatically calculates the bumps from the `bump` keywords above 
    // and passes them here as a struct so we can save them.
    pub fn intialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        // Save the bump for the vault into our VaultState account.
        self.vault_state.vault_bump = bumps.vault;
        
        // Save the bump for the vault_state into our VaultState account.
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}