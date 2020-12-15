use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AsmError {
    #[error("Invalid command type: {0}")]
    InvalidCommandType(String),
    #[error("Invalid mnemonic dest: {0}")]
    InvalidMnemonicDest(String),
    #[error("Invalid mnemonic comp: {0}")]
    InvalidMnemonicComp(String),
    #[error("Invalid mnemonic jump: {0}")]
    InvalidMnemonicJump(String),
}