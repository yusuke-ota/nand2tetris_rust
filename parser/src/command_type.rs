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
            // todo:　.vm内のものに変更
            "C_Label" => Ok(CommandType::CLabel),
            "C_Goto" => Ok(CommandType::CGoto),
            "C_If" => Ok(CommandType::CIf),
            "C_Function" => Ok(CommandType::CFunction),
            "C_Return" => Ok(CommandType::CReturn),
            "C_Call" => Ok(CommandType::CCall),
            _ => Err("Cannot convert &str to CommandType"),
        }
    }
}
