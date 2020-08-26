use crate::{CodeWriter, ICodeWriter};
use parser::arithmetic_type::ArithmeticType;
use parser::command_type::CommandType;
use parser::Parser;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;
use std::mem::swap;

impl ICodeWriter for CodeWriter {
    fn new(path: &str) -> Self {
        let file = File::create(path).expect("Create file failed.");
        Self {
            export_dir: file,
            write_buffer: Vec::<u8>::new(),
            parsers: Vec::<Parser>::new(),
        }
    }

    fn set_file_name(&mut self, file_name: &str) {
        let new_file = File::open(file_name)
            .expect("set_file_name: `File::open()` was failed. Maybe such file don't exist");
        self.parsers.push(Parser::new(new_file));
    }

    fn write_arithmetic(&mut self, command: &str) {
        let arithmetic_type = ArithmeticType::try_from(command).unwrap_or_else(|err| panic!(err));
        self.write_buffer.append(&mut arithmetic_type.into());
    }

    fn write_push_pop(&mut self, command: CommandType, segment: String, index: u32) {
        let assemble_code = format!("{} {} {}\n", <&'static str>::from(command), segment, index);
        let mut buffer = assemble_code
            .as_bytes()
            .iter()
            .copied()
            .collect::<Vec<u8>>();
        self.write_buffer.append(&mut buffer);
    }

    fn close(&mut self) {
        // self.export_dir.write_all(&self.write_buffer) cannot compile
        // because mutable reference and immutable reference was used in one line.
        let mut write_buffer = Vec::<u8>::new();
        swap(&mut write_buffer, &mut self.write_buffer);

        self.export_dir
            .write_all(&write_buffer)
            .expect("close(): `write_all()` was failed.");
        self.export_dir
            .flush()
            .expect("close(): `.flush()` was failed. Are you using this file?");
    }
}
