mod arithmetic_assembly;
mod code_writer;
mod command_assembly;

use parser::command_type::CommandType;
use std::fs::File;

pub struct CodeWriter {
    file_name: Option<String>,
    export_dir: File,
    write_buffer: Vec<u8>,
    label_number: u32,
}

pub trait CodeWriterPublicAPI {
    fn new(path: &str) -> Self;
    fn set_file_name(&mut self, file_name: String);
    fn write_arithmetic(&mut self, command: &str);
    fn write_push_pop(&mut self, command: CommandType, segment: String, index: u32);
    fn close(&mut self);
}

trait CommandAsAssembly {
    fn as_assembly(&self, filename: &str, segment: String, index: u32) -> Vec<u8>;
}

trait ArithmeticAsAssembly {
    fn as_assembly(&self, label_num: &mut u32) -> Vec<u8>;
}
