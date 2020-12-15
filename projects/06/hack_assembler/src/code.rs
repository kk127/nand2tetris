use crate::error::AsmError;

pub fn dest(mnemonic: &str) -> Result<&str,AsmError> {
    match mnemonic {
        ""    => Ok("000"),
        "M"   => Ok("001"),
        "D"   => Ok("010"),
        "MD"  => Ok("011"),
        "A"   => Ok("100"),
        "AM"  => Ok("101"),
        "AD"  => Ok("110"),
        "AMD" => Ok("111"),
        _ => Err(AsmError::InvalidMnemonicDest(mnemonic.to_string()))
    }
}

pub fn comp(mnemonic: &str) -> Result<&str, AsmError> {
    match mnemonic {
        "0"   => Ok("101010"),
        "1"   => Ok("111111"),
        "-1"  => Ok("111010"),
        "D"   => Ok("001100"),
        "A"   => Ok("110000"),
        "!D"  => Ok("001101"),
        "!A"  => Ok("110001"),
        "-D"  => Ok("001111"),
        "-A"  => Ok("110011"),
        "D+1" => Ok("011111"),
        "A+1" => Ok("110111"),
        "D-1" => Ok("001110"),
        "A-1" => Ok("110010"),
        "D+A" => Ok("000010"),
        "D-A" => Ok("010011"),
        "A-D" => Ok("000111"),
        "D&A" => Ok("000000"),
        "D|A" => Ok("010101"),
        "M"   => Ok("110000"),
        "!M"  => Ok("110001"),
        "-M"  => Ok("110011"),
        "M+1" => Ok("110111"),
        "M-1" => Ok("110010"),
        "D+M" => Ok("000010"),
        "D-M" => Ok("010011"),
        "M-D" => Ok("000111"),
        "D&M" => Ok("000000"),
        "D|M" => Ok("010101"),
        _ => Err(AsmError::InvalidMnemonicComp(mnemonic.to_string())),
        
    }
}

pub fn jump(mnemonic: &str) -> Result<&str, AsmError> {
    match mnemonic {
        "" => Ok("000"),
        "JGT" => Ok("001"),
        "JEQ" => Ok("010"),
        "JGE" => Ok("011"),
        "JLT" => Ok("100"),
        "JNE" => Ok("101"),
        "JLE" => Ok("110"),
        "JMP" => Ok("111"),
        _ => Err(AsmError::InvalidMnemonicJump(mnemonic.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dest() {
        assert_eq!(dest(""),    Ok("000"));
        assert_eq!(dest("M"),   Ok("001"));
        assert_eq!(dest("D"),   Ok("010"));
        assert_eq!(dest("MD"),  Ok("011"));
        assert_eq!(dest("A"),   Ok("100"));
        assert_eq!(dest("AM"),  Ok("101"));
        assert_eq!(dest("AD"),  Ok("110"));
        assert_eq!(dest("AMD"), Ok("111"));
        assert_eq!(dest("error"), Err(AsmError::InvalidMnemonicDest("error".to_string())));
    }

    #[test]
    fn test_comp() {
        assert_eq!(comp("0"),   Ok("101010"));
        assert_eq!(comp("1"),   Ok("111111"));
        assert_eq!(comp("-1"),  Ok("111010"));
        assert_eq!(comp("D"),   Ok("001100"));
        assert_eq!(comp("A"),   Ok("110000"));
        assert_eq!(comp("!D"),  Ok("001101"));
        assert_eq!(comp("!A"),  Ok("110001"));
        assert_eq!(comp("-D"),  Ok("001111"));
        assert_eq!(comp("-A"),  Ok("110011"));
        assert_eq!(comp("D+1"), Ok("011111"));
        assert_eq!(comp("A+1"), Ok("110111"));
        assert_eq!(comp("D-1"), Ok("001110"));
        assert_eq!(comp("A-1"), Ok("110010"));
        assert_eq!(comp("D+A"), Ok("000010"));
        assert_eq!(comp("D-A"), Ok("010011"));
        assert_eq!(comp("A-D"), Ok("000111"));
        assert_eq!(comp("D&A"), Ok("000000"));
        assert_eq!(comp("D|A"), Ok("010101"));
        assert_eq!(comp("M"),   Ok("110000"));
        assert_eq!(comp("!M"),  Ok("110001"));
        assert_eq!(comp("-M"),  Ok("110011"));
        assert_eq!(comp("M+1"), Ok("110111"));
        assert_eq!(comp("M-1"), Ok("110010"));
        assert_eq!(comp("D+M"), Ok("000010"));
        assert_eq!(comp("D-M"), Ok("010011"));
        assert_eq!(comp("M-D"), Ok("000111"));
        assert_eq!(comp("D&M"), Ok("000000"));
        assert_eq!(comp("D|M"), Ok("010101"));
        assert_eq!(comp("error"), Err(AsmError::InvalidMnemonicComp("error".to_string())));
    }

    #[test]
    fn test_jump() {
        assert_eq!(jump(""),    Ok("000"));
        assert_eq!(jump("JGT"), Ok("001"));
        assert_eq!(jump("JEQ"), Ok("010"));
        assert_eq!(jump("JGE"), Ok("011"));
        assert_eq!(jump("JLT"), Ok("100"));
        assert_eq!(jump("JNE"), Ok("101"));
        assert_eq!(jump("JLE"), Ok("110"));
        assert_eq!(jump("JMP"), Ok("111"));
        assert_eq!(jump("error"), Err(AsmError::InvalidMnemonicJump("error".to_string())));
    }
}
