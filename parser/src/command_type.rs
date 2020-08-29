use std::convert::TryFrom;

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
        return match value {
            "C_Arithmetic" => Ok(CommandType::CArithmetic),
            "C_Push" => Ok(CommandType::CPush),
            "C_Pop" => Ok(CommandType::CPop),
            "C_Label" => Ok(CommandType::CLabel),
            "C_Goto" => Ok(CommandType::CGoto),
            "C_If" => Ok(CommandType::CIf),
            "C_Function" => Ok(CommandType::CFunction),
            "C_Return" => Ok(CommandType::CReturn),
            "C_Call" => Ok(CommandType::CCall),
            _ => Err("Cannot convert &str to CommandType"),
        };
    }
}
