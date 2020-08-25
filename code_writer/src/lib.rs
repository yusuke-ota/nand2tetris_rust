use parser::command_type::CommandType;
struct CodeWriter{

}

trait ICodeWriter{
    fn new(stream: todo!()) -> Self;
    fn set_file_name(&self, file_name: String);
    fn write_arithmetic(&self, command: CommandType);
    fn write_push_pop(&self, command: CommandType);
    fn close(&self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
