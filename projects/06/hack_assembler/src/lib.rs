mod parser;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsmError {
    #[error("Invalid command type")]
    InvalidCommandType(String),
}
