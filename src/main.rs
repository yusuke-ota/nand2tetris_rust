use code_writer::{CodeWriter, CodeWriterPublicAPI};
use parser::command_type::CommandType;
use parser::{Parser, ParserPublicAPI};
use std::env;
use std::fs::{metadata, read_dir, File};

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        panic!("Please input file path")
    }

    let metadata = metadata(&args[1]).expect("Such does not exist.");
    let mut code_writer;
    match metadata.is_dir() {
        false => {
            // Remove extension ".vm".
            // &args[0] is this application name.
            let file_path = args[1].trim_end_matches(".vm");
            code_writer = CodeWriter::new(&format!("{}.asm", file_path));
            code_writer.write_init();
            let mut parser = Parser::new(File::open(&args[1]).expect("Create file failed."));

            process_file(&args[1], &mut code_writer, &mut parser)?;
        }
        true => {
            // UNWRAP: Method.last() is always Some(). Method.last() search iter until None and return latest Some().
            let folder_name = args[1].trim_end_matches('\\').split('\\').last().unwrap();
            code_writer = CodeWriter::new(&format!("{}\\{}.asm", &args[1], folder_name));
            code_writer.write_init();

            let folder = read_dir(&args[1]).expect("Folder does not exist.");
            for something_in_folder in folder {
                if let Ok(file_in_folder) = something_in_folder {
                    // UNWRAP(): file path is utf-8.
                    let file_path = file_in_folder.path().to_string_lossy().to_string();
                    if file_path.contains(".vm") {
                        // UNWRAP(): Checked at `dir_entry?`
                        let mut parser = Parser::new(File::open(&file_path).unwrap());
                        process_file(&file_path, &mut code_writer, &mut parser)?;
                    }
                }
            }
        }
    }

    code_writer.close();
    Ok(())
}

fn process_file(file_path: &str, code_writer: &mut CodeWriter, parser: &mut Parser) -> anyhow::Result<()> {
    let file_name = file_path.split('\\').last().unwrap().trim_end_matches(".vm").to_string();
    code_writer.set_file_name(file_name);

    while parser.has_more_commands() {
        parser.advance();
        process_command(parser, code_writer)?;
    }
    Ok(())
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
