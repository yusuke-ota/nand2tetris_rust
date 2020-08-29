mod arithmetic_assembly;
mod code_writer;
mod command_assembly;

use parser::command_type::CommandType;
use parser::Parser;
use std::fs::File;

struct CodeWriter {
    file_name: String,
    export_dir: File,
    write_buffer: Vec<u8>,
    label_number: u32,
    parsers: Vec<Parser>,
}

trait ICodeWriter {
    fn new(path: &str) -> Self;
    fn set_file_name(&mut self, file_name: &str);
    fn write_arithmetic(&mut self, command: &str);
    fn write_push_pop(&mut self, command: CommandType, segment: String, index: u32);
    fn close(&mut self);
}

trait CommandAssemblyGenerator {
    fn as_assembly(&self, filename: &str, segment: String, index: u32) -> Vec<u8>;
}

trait ArithmeticAssemblyGenerator {
    fn as_assembly(&self, label_num: &mut u32) -> Vec<u8>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
