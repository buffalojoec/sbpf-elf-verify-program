//! Program instructions.

use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Instructions supported by the BPF Verifier program.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BpfVerifierInstruction {
    /// Verify a program ELF.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[ ]` Buffer account.
    /// 1. `[ ]` ... Will probably need more in the future.
    Verify,
}

impl BpfVerifierInstruction {
    /// Packs a
    /// [BpfVerifierInstruction](enum.BpfVerifierInstruction.html)
    /// into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        match self {
            Self::Verify => vec![0],
        }
    }

    /// Unpacks a byte buffer into a
    /// [BpfVerifierInstruction](enum.BpfVerifierInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input.split_first() {
            Some((&0, _)) => Ok(Self::Verify),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

/// Creates a
/// [Verify](enum.BpfVerifierInstruction.html)
/// instruction.
pub fn verify(buffer_address: &Pubkey) -> Instruction {
    let accounts = vec![AccountMeta::new_readonly(*buffer_address, false)];
    let data = BpfVerifierInstruction::Verify.pack();
    Instruction::new_with_bytes(crate::id(), &data, accounts)
}
