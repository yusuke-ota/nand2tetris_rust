use std::convert::TryFrom;

pub enum ArithmeticType{
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

impl TryFrom<&str> for ArithmeticType{
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
            _ => Err("Convert error")
        }
    }
}

impl From<ArithmeticType> for &'static str{
    fn from(arithmetic_type: ArithmeticType) -> Self {
        return match arithmetic_type {
            // todo: 変換先のアセンブリを記載
            ArithmeticType::Add => "D=D+A\n@0\nM=D",
            ArithmeticType::Sub => "D=D-A\n@0\nM=D",
            ArithmeticType::Neg => "",
            ArithmeticType::Eq => "",
            ArithmeticType::Gt => "",
            ArithmeticType::Lt => "",
            ArithmeticType::And => "",
            ArithmeticType::Or => "",
            ArithmeticType::Not => "",
        }
    }
}