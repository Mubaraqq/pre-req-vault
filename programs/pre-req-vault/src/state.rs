// ==========================================================
// FILE: state.rs - first file
// PURPOSE: Defines the data structure (blueprint) for the program's accounts.
// This file contains no logic (no deposits, no withdrawals). It simply 
// tells the program what a "VaultState" looks like when stored on-chain.
// ==========================================================

// Brings in Anchor's standard library (macros, types, traits).
// Without this, the compiler wouldn't know what #[account] or #[derive] means.
use anchor_lang::prelude::*;

// This macro automatically calculates the exact number of bytes 
// this struct will take up in memory. 
// We use this later in the initialize function to tell Solana exactly 
// how much rent (SOL) to charge for creating this account on-chain.
#[derive(InitSpace)]

// This macro turns the Rust struct below into an actual Solana Account.
// When Anchor sees this, it automatically adds an 8-byte "discriminator" 
// to the front of the data. This is how the program knows: 
// "Ah, this is a VaultState account, not some other account."
#[account]
pub struct VaultState {
    // A "bump" is a single byte (0-255) used to force the PDA off the elliptic curve.
    // We store it here so we don't have to recalculate it every time we need 
    // to sign a transaction with the vault PDA.
    pub vault_bump: u8,
    
    // The exact same thing, but for the "state" PDA. 
    // Every PDA needs its bump saved so it can sign for itself later.
    pub state_bump: u8,
}