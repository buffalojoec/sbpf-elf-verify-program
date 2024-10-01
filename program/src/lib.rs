//! Solana BPF Verifier Program.
//!
//! On-chain BPF program for verifying program ELFs.
#![allow(unexpected_cfgs)]

#[cfg(all(target_os = "solana", feature = "bpf-entrypoint"))]
mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;

solana_program::declare_id!("BPFVerifier111111111111111111111111111111111");
