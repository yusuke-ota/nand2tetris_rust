use asembler::tools::{parser::Parser, *};
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        panic!("Please input file path")
    }
    // &args[0]はこのバイナリの名前が入る
    // &args[0] is this application name.
    let file = File::open(&args[1]).unwrap_or_else(|_| panic!("not found: {:?}", args));
    let mut parser = Parser::new(file);

    let mut symbol_table = SymbolTable::new();
    // シンボルテーブルに登録する
    // Register symbol table
    {
        let mut parser_for_symbol = parser.clone();
        let mut symbol_counter = 16;
        let mut line_counter = 0;
        while parser_for_symbol.has_more_commands() {
            parser.advance();
            line_counter += 1;
            let command_type = parser.command_type();
            // Pattern match and create machine code.
            match command_type {
                // a command: @xxx (xxx: i32), @xxx (xxx: &str)
                // When a command is @xxx (xxx: &str) type, add to symbol_table.
                CommandType::ACommand(_) => {
                    if let Symbol::Symbol(symbol) = parser.symbol() {
                        if symbol_table.contains(&symbol) {
                            continue;
                        };

                        symbol_table.add_entry(symbol, symbol_counter);
                        symbol_counter += 1;
                    }
                }
                // l command: (xxx) (xxx: &str)
                // "line_counter" is counter counts .asm file line without L Command, empty line and comment line.
                CommandType::LCommand(_) => {
                    if let Symbol::Symbol(symbol) = parser.symbol() {
                        if symbol_table.contains(&symbol) {
                            continue;
                        };

                        symbol_table.add_entry(symbol, line_counter);
                        line_counter -= 1;
                    }
                }
                _ => continue,
            }
        }
    }

    // make output string
    let mut write_string = String::new();
    while parser.has_more_commands() {
        parser.advance();
        let command_type = parser.command_type();
        // Pattern match and create machine code.
        match command_type {
            CommandType::CCommand(_) => {
                write_string.push_str(&make_c_command_machine_code(&parser))
            }
            CommandType::ACommand(_) | CommandType::LCommand(_) => match parser.symbol().unwrap() {
                Symbol::Address(num) => write_string.push_str(&format!("{:016b}", num)),
                Symbol::Symbol(string) => {
                    let num = symbol_table
                        .get_address(&string)
                        .unwrap_or_else(|err| panic!(err));
                    write_string.push_str(&format!("{:016b}", num))
                }
            },
        }
    }
    write_string.push_str("\n");

    // 拡張子(.asm)を削除
    // Remove extension ".asm".
    let asm_extension: &[_] = &['.', 'a', 's', 'm'];
    let mut output_file = File::create(format!("{}.hack", args[1].trim_end_matches(asm_extension)))
        .unwrap_or_else(|_| panic!("same file exist"));
    output_file
        .write_all(write_string.as_bytes())
        .unwrap_or_else(|_| panic!("write is failed"));
}

fn make_c_command_machine_code(parser: &Parser) -> String {
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
    header
        .chars()
        .chain(comp.chars())
        .chain(dest.chars())
        .chain(jump.chars())
        .collect::<String>()
}
