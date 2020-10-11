use code_writer::{CodeWriter, CodeWriterPublicAPI};
use parser::command_type::CommandType;
use parser::{Parser, ParserPublicAPI};
use std::env;
use std::fs::File;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        panic!("Please input file path")
    }
    // &args[0] is this application name.
    let mut parser = Parser::new(File::open(&args[1]).expect("Create file failed."));

    // Remove extension ".vm".
    let vm_extension: &[_] = &['.', 'v', 'm'];
    let file_path = args[1].trim_end_matches(vm_extension);
    let mut code_writer = CodeWriter::new(&format!("{}.asm", file_path));

    let file_name = file_path.split('\\').last().unwrap().to_string();
    code_writer.set_file_name(file_name);

    while parser.has_more_commands() {
        parser.advance();
        process_command(&parser, &mut code_writer);
    }

    code_writer.close();
}

/// Generate assembly and write to code_writer.
fn process_command(parser: &Parser, code_writer: &mut CodeWriter) {
    match parser.command_type() {
        CommandType::CArithmetic => {
            let arithmetic_type = parser.arg1();
            code_writer.write_arithmetic(arithmetic_type.as_str())
        }
        CommandType::CPush => {
            let segment = parser.arg1();
            let index = parser.arg2();
            code_writer.write_push_pop(CommandType::CPush, segment, index)
        }
        CommandType::CPop => {
            let segment = parser.arg1();
            let index = parser.arg2();
            code_writer.write_push_pop(CommandType::CPop, segment, index)
        }
        CommandType::CLabel => unimplemented!(),
        CommandType::CGoto => unimplemented!(),
        CommandType::CIf => unimplemented!(),
        CommandType::CFunction => unimplemented!(),
        CommandType::CReturn => unimplemented!(),
        CommandType::CCall => unimplemented!(),
    }
}
