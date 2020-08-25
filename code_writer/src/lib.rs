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
    fn set_file_name(&mut self, file_name: &str);
    fn write_arithmetic(&mut self, command: &str);
    fn write_push_pop(&mut self, command: CommandType, segment: String, index: u32);
    fn close(&mut self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
