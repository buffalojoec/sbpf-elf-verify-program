//! Program error types.

use {
    num_traits::{self, FromPrimitive},
    solana_bpf_verifier::VerifyError,
    solana_program::{
        decode_error::DecodeError,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

/// Errors that can be returned by the BPF Verifier program.
#[derive(Debug, Error)]
pub enum BPFVerifierError {
    /// ELF verification error.
    #[error(transparent)]
    VerifyError(#[from] VerifyError),
}

impl FromPrimitive for BPFVerifierError {
    fn from_i64(n: i64) -> Option<Self> {
        VerifyError::from_i64(n).map(BPFVerifierError::VerifyError)
    }

    fn from_u64(n: u64) -> Option<Self> {
        VerifyError::from_u64(n).map(BPFVerifierError::VerifyError)
    }
}

impl From<BPFVerifierError> for ProgramError {
    fn from(e: BPFVerifierError) -> Self {
        match e {
            BPFVerifierError::VerifyError(e) => e.into(),
        }
    }
}

impl<T> DecodeError<T> for BPFVerifierError {
    fn type_of() -> &'static str {
        "TokenError"
    }
}

impl PrintProgramError for BPFVerifierError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            Self::VerifyError(e) => PrintProgramError::print::<VerifyError>(e),
        }
    }
}
