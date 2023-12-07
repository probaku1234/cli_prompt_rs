use std::fmt::{Debug, Display, Formatter};
use std::io;

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

#[derive(Debug)]
pub enum CliPromptError {
    IoError(io::Error),
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
