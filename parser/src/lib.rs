mod parser;
use command_type::CommandType;
use std::fs::File;

pub struct Parser{
    stream: Vec<String>,
    command: Option<String>,
}

trait IParser{
    /// Create Parser from Files.
    fn new(file: File) -> Self;
    fn has_more_commands(&self) -> bool;
    fn advance(&self);
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
