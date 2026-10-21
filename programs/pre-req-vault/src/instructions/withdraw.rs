// ==========================================================
// FILE: instructions/withdraw.rs file 6
// PURPOSE: Allows a user to withdraw SOL from the Vault. 
// Crucially, it also triggers a Cross-Program Invocation (CPI) 
// to the external Registration Program to save their GitHub username.
// ==========================================================

use crate::state::VaultState;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

// This macro looks at the IDL file (`idls/registration.json`) 
// and automatically generates the Rust code needed to talk to that program.
declare_program!(registration);

// We import the specific function (`initialize`) and its account struct 
// (`Initialize`) from the generated registration CPI module.
use registration::cpi::{accounts::Initialize, initialize};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // The Vault PDA. `mut` because SOL is leaving it.
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    // The new account we need for the CPI.
    // `UncheckedAccount` means Anchor won't try to deserialize its data 
    // (because it doesn't exist yet, or we don't care about its data).
    // `seeds::program = application_program.key()` is the magic here. 
    // It tells Anchor: "Don't use OUR program ID to derive this PDA, 
    // use the Registration program's ID."
    /// CHECK: application account will be initialized by the cpi call to the application program
    #[account(
        mut,
        seeds = [b"prereqs", user.key().as_ref()],
        seeds::program = application_program.key(),
        bump
    )]
    pub application_account: UncheckedAccount<'info>,

    // The external Registration program itself.
    pub application_program: Program<'info, registration::program::Q3PreReqsRs>,

    system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64, github: String) -> Result<()> {
        
        // --- PART 1: WITHDRAW SOL ---
        
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        // Because the Vault is a PDA, it doesn't have a private key to sign with.
        // We have to reconstruct the seeds and the bump to "sign" on its behalf.
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // Notice `new_with_signer` instead of `new`. We are passing the PDA seeds.
        let cpi_ctx = CpiContext::new_with_signer(System::id(), cpi_accounts, signer_seeds);

        // Execute the transfer (Vault -> User)
        transfer(cpi_ctx, amount)?;

        // --- PART 2: CPI TO REGISTRATION PROGRAM ---

        // Set up the context for the external program.
        // We use `new` (not `new_with_signer`) because the User is signing this, 
        // not the Vault PDA.
        let reg_cpi_ctx = CpiContext::new(
            self.application_program.key(),
            Initialize {
                user: self.user.to_account_info(),
                // The IDL calls it "account", our struct calls it "application_account"
                account: self.application_account.to_account_info(), 
                system_program: self.system_program.to_account_info(),
            },
        );

        // Execute the CPI, passing in the GitHub username string.
        initialize(reg_cpi_ctx, github)?;

        Ok(())
    }
}