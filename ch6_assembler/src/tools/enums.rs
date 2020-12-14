use std::convert::TryFrom;

pub enum CommandType {
    ACommand(String),
    CCommand(String),
    LCommand(String),
}

pub enum Symbol {
    Address(i32),
    Symbol(String),
}

pub fn classification_symbol(symbol: &str) -> Symbol {
    match symbol.parse::<i32>() {
        Ok(num) => Symbol::Address(num),
        Err(_) => Symbol::Symbol(symbol.to_string()),
    }
}

pub enum DestType {
    Null,
    M,
    D,
    MD,
    A,
    AM,
    AD,
    AMD,
}

impl TryFrom<&str> for DestType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "0" => Ok(DestType::Null),
            "M" => Ok(DestType::M),
            "D" => Ok(DestType::D),
            "MD" => Ok(DestType::MD),
            "A" => Ok(DestType::A),
            "AM" => Ok(DestType::AM),
            "AD" => Ok(DestType::AD),
            "AMD" => Ok(DestType::AMD),
            _ => Err("Cannot parse to DestType"),
        }
    }
}

pub enum JumpType {
    Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

impl TryFrom<&str> for JumpType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "null" => Ok(JumpType::Null),
            "JGT" => Ok(JumpType::JGT),
            "JEQ" => Ok(JumpType::JEQ),
            "JGE" => Ok(JumpType::JGE),
            "JLT" => Ok(JumpType::JLT),
            "JNE" => Ok(JumpType::JNE),
            "JLE" => Ok(JumpType::JLE),
            "JMP" => Ok(JumpType::JMP),
            _ => Err("Jump parse error!"),
        }
    }
}

pub enum CompType {
    Zero,
    One,
    MinusOne,
    D,
    A,
    NotD,
    NotA,
    MinusD,
    MinusA,
    DPlusOne,
    APlusOne,
    DMinusOne,
    AMinusOne,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
    M,
    NotM,
    MinusM,
    MPlusOne,
    MMinusOne,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

impl TryFrom<&str> for CompType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "0" => Ok(CompType::Zero),
            "1" => Ok(CompType::One),
            "-1" => Ok(CompType::MinusOne),
            "D" => Ok(CompType::D),
            "A" => Ok(CompType::A),
            "!D" => Ok(CompType::NotD),
            "!A" => Ok(CompType::NotA),
            "-D" => Ok(CompType::MinusD),
            "-A" => Ok(CompType::MinusA),
            "D+1" => Ok(CompType::DPlusOne),
            "A+1" => Ok(CompType::APlusOne),
            "D-1" => Ok(CompType::DMinusOne),
            "A-1" => Ok(CompType::AMinusOne),
            "D+A" => Ok(CompType::DPlusA),
            "D-A" => Ok(CompType::DMinusA),
            "A-D" => Ok(CompType::AMinusD),
            "D&A" => Ok(CompType::DAndA),
            "D|A" => Ok(CompType::DOrA),
            "M" => Ok(CompType::M),
            "!M" => Ok(CompType::NotM),
            "-M" => Ok(CompType::MinusM),
            "M+1" => Ok(CompType::MPlusOne),
            "M-1" => Ok(CompType::MMinusOne),
            "D+M" => Ok(CompType::DPlusM),
            "D-M" => Ok(CompType::DMinusM),
            "M-D" => Ok(CompType::MMinusD),
            "D&M" => Ok(CompType::DAndM),
            "D|M" => Ok(CompType::DOrM),
            _ => Err("Unexpected value of Comp"),
        }
    }
}
