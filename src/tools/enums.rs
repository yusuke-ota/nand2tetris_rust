pub enum CommandType{
    ACommand(String),
    CCommand(String),
    LCommand(String)
}

pub enum Symbol{
    Address(i32),
    Symbol(String)
}

pub fn classification_symbol(symbol: &str) -> Symbol{
    return match symbol.parse::<i32>() {
        Ok(num) => Symbol::Address(num),
        Err(_) => Symbol::Symbol(symbol.to_string())
    }
}

pub enum DestType{
    Null,
    M,
    D,
    MD,
    A,
    AM,
    AD,
    AMD,
}

pub enum JumpType{
    Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

pub enum CompType{
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
    DOrM
}
