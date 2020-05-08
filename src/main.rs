use std::env;
use std::fs::File;
use std::io::Write;
use asembler::tools::parser::Parser;
use asembler::tools::enums::*;
use std::ops::Add;
use asembler::tools::*;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file = File::open(&args[0]).unwrap_or_else(panic!("not found: {:?}", args));
    let mut parser = Parser::new(file);

    let mut write_string = String::new();
    while parser.has_more_commands() {
        parser.advance();
        let command_type = parser.command_type();
        match command_type {
            command_type if command_type == CommandType::ACommand || command_type == CommandType::LCommand =>
                match parser.symbol().unwrap() {
                    Symbol::Address(num) => write_string.add(num as &str),
                    Symbol::Symbol(string) => write_string.add(string as &str)
                },
            command_type if command_type == CommandType::CCommand => {
                let comp = comp(parser.comp().unwrap_or(CompType::Zero));
                let dest = dest(parser.dest().unwrap_or(DestType::Null));
                let jump = jump(parser.jump().unwrap_or(JumpType::Null));
                let add_string = comp.iter().chain(dest.iter()).chain(jump.iter()).collect::<String>();
                write_string.add(&add_string)
            },
        }
        write_string.add("\n");
    }
    let mut output_file = File::create(format!("{}.hack", args[0])).unwrap_or_else(panic!("same file exist"));
    output_file.write_all(write_string.as_bytes()).unwrap_or_else(panic!("write is failed"));
}
