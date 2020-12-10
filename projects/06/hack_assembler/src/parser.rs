use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AsmError {
    #[error("Invalid command type")]
    InvalidCommandType(String),
}

#[derive(Debug, PartialEq)]
enum CommandType {
    ACommand,
    CCommand,
    LCommand,
}

#[derive(Debug, PartialEq)]
struct Parser {
    index: usize,
    asm: Vec<String>,
    current_command: String,
}

impl Parser {
    // コンストラクタ
    fn new(path: &Path) -> Self {
        // ファイル読み込み

        let mut asm = Vec::new();
        let file = File::open(path).expect("cann't open file");
        let buffer = BufReader::new(file);
        for line in buffer.lines() {
            let read_line = line.expect("cann't read line");
            asm.push(read_line)
        }

        Parser {
            index: 0,
            asm: asm,
            current_command: "".to_string(),
        }
    }

    // 入力にコマンドが存在するか
    fn has_more_command(&self) -> bool {
        self.index < self.asm.len()
    }

    fn advance(&mut self) {
        self.current_command = self.asm[self.index].clone();
        self.index += 1;
    }

    fn command_type(&self) -> Result<CommandType, AsmError> {
        let re_a = Regex::new(r"^@\d*$|^@[[:alpha:].$:][[:word:].$:]*$").unwrap();
        let re_l = Regex::new(r"^\(\d*\)$|^\([[:alpha:].$:][[:word:].$:]*\)$").unwrap();
        println!("{}", self.current_command);
        if re_a.is_match(&self.current_command) {
            return Ok(CommandType::ACommand);
        } else if re_l.is_match(&self.current_command) {
            return Ok(CommandType::LCommand);
        } else {
            return Err(AsmError::InvalidCommandType("invalid".to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let path = Path::new("./src/input/test.txt");
        let parser = Parser::new(path);

        let correct_parser = Parser {
            index: 0,
            asm: vec!["000".to_string(), "111".to_string(), "222".to_string()],
            current_command: "".to_string(),
        };

        assert_eq!(parser, correct_parser);
    }

    #[test]
    fn test_has_command() {
        let path = Path::new("./src/input/test.txt");
        let parser = Parser::new(path);
        assert_eq!(parser.has_more_command(), true);

        let empty_parser = Parser {
            index: 0,
            asm: vec![],
            current_command: "".to_string(),
        };
        assert_eq!(empty_parser.has_more_command(), false);
    }

    #[test]
    fn test_advance() {
        let path = Path::new("./src/input/test.txt");
        let mut parser = Parser::new(path);
        parser.advance();
        assert_eq!(parser.current_command, "000");
        parser.advance();
        assert_eq!(parser.current_command, "111");
        parser.advance();
        assert_eq!(parser.current_command, "222");
    }

    #[test]
    fn test_commandA_type() {
        let path = Path::new("./src/input/commandA.txt");
        let mut parser = Parser::new(path);

        parser.advance();
        assert_eq!(parser.command_type(), Ok(CommandType::ACommand));
        parser.advance();
        assert_eq!(parser.command_type(), Ok(CommandType::ACommand));
        parser.advance();
        assert_eq!(
            parser.command_type(),
            Err(AsmError::InvalidCommandType("invalid".to_string()))
        );
    }

    #[test]
    fn test_commandL_type() {
        let path = Path::new("./src/input/commandL.txt");
        let mut parser = Parser::new(path);

        parser.advance();
        assert_eq!(parser.command_type(), Ok(CommandType::LCommand));
        parser.advance();
        assert_eq!(parser.command_type(), Ok(CommandType::LCommand));
        parser.advance();
        assert_eq!(parser.command_type(), Ok(CommandType::LCommand));
        parser.advance();
        assert_eq!(
            parser.command_type(),
            Err(AsmError::InvalidCommandType("invalid".to_string()))
        );
        parser.advance();
        assert_eq!(
            parser.command_type(),
            Err(AsmError::InvalidCommandType("invalid".to_string()))
        );
    }
}
