//! The error type for `CliPrompt` struct
//!
//! All public methods in `CliPrompt` returns `Result` with [`CliPromptError`]
use std::fmt::{Debug, Display, Formatter};
use std::io;
use crate::CliPrompt;
// TODO: add doc
// TODO: add spinner error
// #[derive(Debug)]
// pub struct OptionsVecEmptyError {
//     pub message: String,
// }
//
// impl Display for OptionsVecEmptyError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.message)
//     }
// }

// impl std::error::Error for OptionsVecEmptyError {}

/// A list specifying general categories of error from `CliPrompt` module.
#[derive(Debug)]
pub enum CliPromptError {
    /// Indicates an underlying IO Error.
    IoError(io::Error),
    /// The options vec is empty. Used for [`CliPrompt::prompt_select`], [`CliPrompt::prompt_multi_select`]
    OptionsVecEmptyError { message: String },
}

impl From<io::Error> for CliPromptError {
    fn from(error: io::Error) -> Self {
        CliPromptError::IoError(error)
    }
}

impl Display for CliPromptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CliPromptError::IoError(io_error) => write!(f, "{}", io_error),
            CliPromptError::OptionsVecEmptyError { message } => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for CliPromptError {}
