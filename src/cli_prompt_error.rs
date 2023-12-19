//! The error type for `CliPrompt` struct
//!
//! All public methods in `CliPrompt` returns `Result` with [`CliPromptError`]
use crate::CliPrompt;
use std::fmt::{Debug, Display, Formatter};
use std::io;
// TODO: add doc

#[derive(Debug)]
pub enum SpinnerError {
    TimedOut,
    TaskFailed,
}

/// A list specifying general categories of error from `CliPrompt` module.
#[derive(Debug)]
pub enum CliPromptError {
    /// Indicates an underlying IO Error.
    IoError(io::Error),
    /// The options vec is empty. Used for [`CliPrompt::prompt_select`], [`CliPrompt::prompt_multi_select`]
    OptionsVecEmptyError { message: String },
    /// Used for [`CliPrompt::prompt_multi_select_with_max_choice_num`]
    InvalidMaxChoiceNumError { message: String },
    SpinnerError(SpinnerError)
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
            CliPromptError::InvalidMaxChoiceNumError { message } => write!(f, "{}", message),
            CliPromptError::SpinnerError(spinner_error) => spinner_error.fmt(f),
        }
    }
}

impl std::error::Error for CliPromptError {}
