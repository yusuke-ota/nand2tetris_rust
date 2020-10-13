use crate::{ArithmeticAsAssembly, CodeWriter, CodeWriterPublicAPI, CommandAsAssembly};
use parser::arithmetic_type::ArithmeticType;
use parser::command_type::CommandType;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;
use std::mem::swap;

impl CodeWriterPublicAPI for CodeWriter {
    fn new(file_name: &str) -> Self {
        let file = File::create(file_name).expect("Create file failed.");
        Self {
            file_name: None,
            export_dir: file,
            write_buffer: Vec::<u8>::new(),
            label_number: 0,
            recent_function: None,
        }
    }

    fn write_init(&mut self) {
        let init_stack_pointer =
            "@256\n\
            D=A\n\
            @SP\n\
            M=D\n";
        self.write_buffer.append(init_stack_pointer.as_bytes().to_vec().as_mut());
        let mut assemble_code = CommandType::CCall.as_assembly(self, "Sys.init".to_string(), 0);
        self.write_buffer.append(&mut assemble_code);
    }

    fn set_file_name(&mut self, file_name: String) {
        self.file_name = Some(file_name);
    }

    fn write_arithmetic(&mut self, command: &str) -> anyhow::Result<()> {
        let arithmetic_type = ArithmeticType::try_from(command)?;
        self.write_buffer
            .append(&mut arithmetic_type.as_assembly(&mut self.label_number));
        Ok(())
    }

    fn write_push_pop(&mut self, command: CommandType, segment: String, index: u32) {
        let mut assemble_code = command.as_assembly(self, segment, index);
        self.write_buffer.append(&mut assemble_code);
    }

    fn write_label(&mut self, label: String) {
        // "" and 0 are not used, label only.
        let mut assemble_code = CommandType::CLabel.as_assembly(self, label, 0);
        self.write_buffer.append(&mut assemble_code);
    }

    fn write_goto(&mut self, label: String) {
        // "" and 0 are not used, label only.
        let mut assemble_code = CommandType::CGoto.as_assembly(self, label, 0);
        self.write_buffer.append(&mut assemble_code);
    }

    fn write_if(&mut self, label: String) {
        // "" and 0 are not used, label only.
        let mut assemble_code = CommandType::CIf.as_assembly(self, label, 0);
        self.write_buffer.append(&mut assemble_code);
    }

    fn write_call(&mut self, function_name: String, num_args: u32) {
        let mut assemble_code = CommandType::CCall.as_assembly(self, function_name, num_args);
        self.write_buffer.append(&mut assemble_code);
    }

    fn write_return(&mut self) {
        let mut assemble_code = CommandType::CReturn.as_assembly(self, String::default(), 0);
        self.write_buffer.append(&mut assemble_code);
    }

    fn write_function(&mut self, function_name: String, num_locals: u32) {
        let mut assemble_code = CommandType::CFunction.as_assembly(self, function_name, num_locals);
        self.write_buffer.append(&mut assemble_code);
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
