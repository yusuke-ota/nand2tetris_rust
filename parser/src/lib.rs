pub mod arithmetic_type;
pub mod command_type;
pub mod parser;

use crate::command_type::CommandType;

pub struct Parser {
    stream: Vec<String>,
    command: Option<String>,
}

pub trait IParser {
    fn has_more_commands(&self) -> bool;
    fn advance(&mut self);
    fn command_type(&self) -> CommandType;
    /// This function shouldn't call when Parser::command is CReturn.
    fn arg1(&self) -> String;
    /// This function should call when Parser::command is
    /// * CPush
    /// * CPop
    /// * CFunction
    /// * CCall
    fn arg2(&self) -> i32;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
