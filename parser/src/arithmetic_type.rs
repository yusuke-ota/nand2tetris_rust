use std::convert::TryFrom;

pub enum ArithmeticType {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl TryFrom<&str> for ArithmeticType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "add" => Ok(Self::Add),
            "sub" => Ok(Self::Sub),
            "neg" => Ok(Self::Neg),
            "eq" => Ok(Self::Eq),
            "gt" => Ok(Self::Gt),
            "lt" => Ok(Self::Lt),
            "and" => Ok(Self::And),
            "or" => Ok(Self::Or),
            "not" => Ok(Self::Not),
            _ => Err("Convert error"),
        }
    }
}
