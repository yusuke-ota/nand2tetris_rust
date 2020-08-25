use crate::{ICodeWriter, CodeWriter};
use std::fs::File;
use std::io::Write;
use std::mem::swap;
use parser::Parser;
use std::convert::TryFrom;
use parser::command_type::CommandType;
use parser::arithmetic_type::ArithmeticType;

impl ICodeWriter for CodeWriter{
    fn new(path: &str) -> Self {
        let file = File::create(path).expect("Create file failed.");
        Self{
            export_dir: file,
            write_buffer: Vec::<u8>::new(),
            parsers: Vec::<Parser>::new(),
        }
    }

    fn set_file_name(&mut self, file_name: &str) {
        let new_file = File::open(file_name).expect("set_file_name: `File::open()` was failed. Maybe such file don't exist");
        self.parsers.push(Parser::new(new_file));
    }

    fn write_arithmetic(&mut self, command: &str) {
        let mut buffer = Vec::<u8>::with_capacity(command.len() + 2);
        let arithmetic_type = ArithmeticType::try_from(command).unwrap_or_else(|err| panic!(err));

        let assemble_code = <&'static str>::from(arithmetic_type).to_string();
        let mut char_byte = [0_u8;1];

        for char in assemble_code.chars(){
            // char is single byte string.
            char.encode_utf8(&mut char_byte);
            buffer.push(char_byte[0]);
            char_byte = [0];
        }

        '\n'.encode_utf8(&mut char_byte);
        buffer.push(char_byte[0]);

        self.write_buffer.append(&mut buffer);
    }

    fn write_push_pop(&mut self, command: CommandType, segment: String, index: u32) {
        let assemble_code = format!("{} {} {}", <&'static str>::from(command), segment, index);
        let mut buffer = Vec::<u8>::with_capacity(assemble_code.len() + 2);
        let mut char_byte = [0_u8;1];

        for char in assemble_code.chars(){
            // char is single byte string.
            char.encode_utf8(&mut char_byte);
            buffer.push(char_byte[0]);
            char_byte = [0];
        }

        '\n'.encode_utf8(&mut char_byte);
        buffer.push(char_byte[0]);

        self.write_buffer.append(&mut buffer);
    }

    fn close(&mut self) {
        // self.export_dir.write_all(&self.write_buffer) cannot compile
        // because mutable reference and immutable reference was used in one line.
        let mut write_buffer = Vec::<u8>::new();
        swap(&mut write_buffer, &mut self.write_buffer);

        self.export_dir.write_all(&write_buffer).expect("close(): `write_all()` was failed.");
        self.export_dir.flush().expect("close(): `.flush()` was failed. Are you using this file?");
    }
}