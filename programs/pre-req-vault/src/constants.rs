// ==========================================================
// FILE: constants.rs - file 2
// PURPOSE: Stores hardcoded, immutable values (like seed strings 
// or max limits) in one place so you don't have to type them out 
// repeatedly across different instruction files.
// ==========================================================

// Brings in Anchor's standard library.
use anchor_lang::prelude::*;

// #[constant] exposes this variable to the IDL so clients (like JS tests) 
// can read it. 
// NOTE: This is leftover boilerplate from the default Anchor template. 
// A clean Vault build would define seeds like:
// pub const VAULT_SEED: &[u8] = b"vault";
#[constant]
pub const COUNTER_SEED: &[u8] = b"counter";

// Another leftover from the template. 
// Likely used for a "Hello World" instruction that sends 1 lamport.
#[constant]
pub const HELLO_WORLD_LAMPORTS: u64 = 1;

// Another template leftover. Sets a max limit for a counter.
#[constant]
pub const MAX_COUNT: u64 = 10;