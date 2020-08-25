mod code_writer;

use std::fs::File;
use parser::Parser;
use parser::command_type::CommandType;

struct CodeWriter{
    export_dir: File,
    write_buffer: Vec<u8>,
    parsers: Vec<Parser>
}

trait ICodeWriter{
    fn new(path: &str) -> Self;
    fn set_file_name(&self, file_name: &str);
    fn write_arithmetic(&self, command: &str);
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
