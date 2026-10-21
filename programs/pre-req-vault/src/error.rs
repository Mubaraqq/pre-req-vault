// ==========================================================
// FILE: error.rs - file 3
// PURPOSE: Defines custom, human-readable errors for the program.
// Instead of the program just failing with a generic "Error 0x1770", 
// it fails with a specific message that the frontend can display.
// ==========================================================

// Brings in Anchor's standard library.
use anchor_lang::prelude::*;

// This macro tells Anchor: "Convert this Rust enum into Solana program errors."
// Anchor automatically assigns a unique error code (like 6000, 6001) to each variant.
#[error_code]
pub enum ErrorCode {
    // #[msg("...")] is the exact string that gets passed to the client 
    // (and logged in the transaction details) if this error is thrown.
    // NOTE: This is leftover from the counter template. 
    #[msg("Only the counter authority can update this counter")]
    Unauthorized,
    
    // Another leftover. If we were building the Vault from scratch, 
    // we would have an error here like `InsufficientFunds`.
    #[msg("Counter has reached the maximum value")]
    CounterOverflow,
}