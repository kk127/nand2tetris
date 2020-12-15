use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AsmError {
    #[error("Invalid command type")]
    InvalidCommandType(String),
}