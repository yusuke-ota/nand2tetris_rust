use asembler::tools::{parser::Parser, *};
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    // &args[0]はこのバイナリの名前が入る
    // &args[0] is this application name.
    let file = File::open(&args[1]).unwrap_or_else(|_| panic!("not found: {:?}", args));
    let mut parser = Parser::new(file);

    // make output string
    let mut write_string = String::new();
    while parser.has_more_commands() {
        parser.advance();
        let command_type = parser.command_type();
        // Pattern match and create machine code.
        match command_type {
            CommandType::ACommand(_) | CommandType::LCommand(_) => match parser.symbol().unwrap() {
                Symbol::Address(num) => write_string.push_str(&format!("{:016b}", num)),
                Symbol::Symbol(string) => write_string.push_str(&string),
            },
            CommandType::CCommand(_) => {
                let header = "111";
                let comp = comp(parser.comp().unwrap_or(CompType::Zero))
                    .iter()
                    .map(|&iter| iter.to_string())
                    .collect::<String>();
                let dest = dest(parser.dest().unwrap_or(DestType::Null))
                    .iter()
                    .map(|&iter| iter.to_string())
                    .collect::<String>();
                let jump = jump(parser.jump().unwrap_or(JumpType::Null))
                    .iter()
                    .map(|&iter| iter.to_string())
                    .collect::<String>();
                let add_string = header
                    .chars()
                    .chain(comp.chars())
                    .chain(dest.chars())
                    .chain(jump.chars())
                    .collect::<String>();
                write_string.push_str(&add_string);
            }
        }
        write_string.push_str("\n");
    }

    // 拡張子(.asm)を削除
    let asm_extension: &[_] = &['.', 'a', 's', 'm'];
    let mut output_file = File::create(format!("{}.hack", args[1].trim_end_matches(asm_extension)))
        .unwrap_or_else(|_| panic!("same file exist"));
    output_file
        .write_all(write_string.as_bytes())
        .unwrap_or_else(|_| panic!("write is failed"));
}
