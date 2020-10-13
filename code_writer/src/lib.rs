mod arithmetic_assembly;
mod code_writer;
mod command_assembly;

use parser::command_type::CommandType;
use std::fs::File;
use parser::anyhow;

pub struct CodeWriter {
    file_name: Option<String>,
    export_dir: File,
    write_buffer: Vec<u8>,
    label_number: u32,
    recent_function: Option<String>,
}

pub trait CodeWriterPublicAPI {
    fn new(path: &str) -> Self;
    fn write_init(&mut self);
    fn set_file_name(&mut self, file_name: String);
    fn write_arithmetic(&mut self, command: &str) -> anyhow::Result<()>;
    fn write_push_pop(&mut self, command: CommandType, segment: String, index: u32);
    fn write_label(&mut self, label: String);
    fn write_goto(&mut self, label: String);
    fn write_if(&mut self, label: String);
    fn write_call(&mut self, function_name: String, num_args: u32);
    fn write_return(&mut self);
    fn write_function(&mut self, function_name: String, num_locals: u32);
    fn close(&mut self);
}

/// Generate assembly from command.
trait CommandAsAssembly {
    fn as_assembly(&self, code_writer: &mut CodeWriter, segment: String, index: u32) -> Vec<u8>;
}

/// Generate assembly from arithmetic command.
trait ArithmeticAsAssembly {
    fn as_assembly(&self, label_num: &mut u32) -> Vec<u8>;
}
