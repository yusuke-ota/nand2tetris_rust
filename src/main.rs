use code_writer::{CodeWriter, CodeWriterPublicAPI};
use parser::command_type::CommandType;
use parser::{Parser, ParserPublicAPI};
use std::env;
use std::fs::File;

fn main() -> anyhow::Result<()> {
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
fn process_command(parser: &Parser, code_writer: &mut CodeWriter) -> anyhow::Result<()> {
    match parser.command_type() {
        CommandType::CArithmetic => {
            let arithmetic_type = parser.arg1();
            code_writer.write_arithmetic(arithmetic_type.as_str())?;
            Ok(())
        }
        CommandType::CPush => {
            let segment = parser.arg1();
            let index = parser.arg2();
            code_writer.write_push_pop(CommandType::CPush, segment, index);
            Ok(())
        }
        CommandType::CPop => {
            let segment = parser.arg1();
            let index = parser.arg2();
            code_writer.write_push_pop(CommandType::CPop, segment, index);
            Ok(())
        }
        CommandType::CLabel => {
            let label = parser.arg1();
            code_writer.write_label(label);
            Ok(())
        }
        CommandType::CGoto => {
            let label = parser.arg1();
            code_writer.write_goto(label);
            Ok(())
        }
        CommandType::CIf => {
            let label = parser.arg1();
            code_writer.write_if(label);
            Ok(())
        }
        CommandType::CFunction => {
            let function_name = parser.arg1();
            let num_locals = parser.arg2();
            code_writer.write_function(function_name, num_locals);
            Ok(())
        }
        CommandType::CReturn => {
            code_writer.write_return();
            Ok(())
        }
        CommandType::CCall => {
            let function_name = parser.arg1();
            let num_args = parser.arg2();
            code_writer.write_call(function_name, num_args);
            Ok(())
        }
    }
}
