use std::convert::TryFrom;

#[derive(Eq, PartialEq, Debug)]
pub enum CommandType {
    CArithmetic,
    CPush,
    CPop,
    CLabel,
    CGoto,
    CIf,
    CFunction,
    CReturn,
    CCall,
}

impl TryFrom<&str> for CommandType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
                Ok(CommandType::CArithmetic)
            }
            "push" => Ok(CommandType::CPush),
            "pop" => Ok(CommandType::CPop),
            "label" => Ok(CommandType::CLabel),
            "goto" => Ok(CommandType::CGoto),
            "if-goto" => Ok(CommandType::CIf),
            "function" => Ok(CommandType::CFunction),
            "return" => Ok(CommandType::CReturn),
            "call" => Ok(CommandType::CCall),
            _ => Err("Cannot convert &str to CommandType"),
        }
    }
}
