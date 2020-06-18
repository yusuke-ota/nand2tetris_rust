use std::env;
use std::fs::File;
use std::io::Write;
use asembler::tools::*;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        panic!("Please input file path")
    }
    // &args[0]はこのバイナリの名前が入る
    // &args[0] is this application name.
    let file = File::open(&args[1]).expect("not found: ");
    let mut parser = Parser::new(file);

    let mut symbol_table = SymbolTable::new();
    // シンボルテーブルに登録する
    // Register l command to symbol table
    register_l_command(parser.clone(), &mut symbol_table);
    // Register a command to symbol table
    register_a_command(parser.clone(), &mut symbol_table);

    // make output string
    let mut write_string = String::new();
    while parser.has_more_commands() {
        parser.advance();
        let command_type = parser.command_type();
        // Pattern match and create machine code.
        match command_type {
            Ok(CommandType::CCommand(_)) => {
                write_string.push_str(&make_c_command_machine_code(&parser))
            }
            Ok(CommandType::ACommand(_)) => match parser.symbol().expect("error: ") {
                Symbol::Address(num) => write_string.push_str(&format!("{:016b}", num)),
                Symbol::Symbol(string) => {
                    let num = symbol_table
                        .get_address(&string)
                        .expect("Failed");
                    write_string.push_str(&format!("{:016b}", num))
                }
            },
            _ => continue,
        }

        write_string.push_str("\n");
    }

    // 拡張子(.asm)を削除
    // Remove extension ".asm".
    let asm_extension: &[_] = &['.', 'a', 's', 'm'];
    let mut output_file = File::create(format!("{}.hack", args[1].trim_end_matches(asm_extension)))
        .expect("The file is locked");
    output_file
        .write_all(write_string.as_bytes())
        .expect("The file is locked");
}

fn make_c_command_machine_code(parser: &Parser) -> String {
    let mut machine_code = String::with_capacity(16);
    // push header
    machine_code.push_str("111");
    // push comp
    for &binary in comp(parser.comp().unwrap_or(CompType::Zero)).iter() {
        machine_code.push_str(&binary.to_string());
    }
    // push dest
    for &binary in dest(parser.dest().unwrap_or(DestType::Null)).iter() {
        machine_code.push_str(&binary.to_string());
    }
    // push jump
    for &binary in jump(parser.jump().unwrap_or(JumpType::Null)).iter() {
        machine_code.push_str(&binary.to_string());
    }
    machine_code
}

fn register_l_command(mut parser: Parser, symbol_table: &mut SymbolTable) {
    let mut line_counter = -1;
    while parser.has_more_commands() {
        parser.advance();
        line_counter += 1;
        let command_type = parser.command_type();
        if let Ok(CommandType::LCommand(_)) = command_type {
            if let Ok(Symbol::Symbol(symbol)) = parser.symbol() {
                if symbol_table.contains(&symbol) {
                    continue;
                };

                symbol_table.add_entry(symbol, line_counter);
                line_counter -= 1;
            }
        }
    }
}

fn register_a_command(mut parser: Parser, symbol_table: &mut SymbolTable) {
    let mut symbol_counter = 16;
    while parser.has_more_commands() {
        parser.advance();
        if let Ok(CommandType::ACommand(_)) = parser.command_type() {
            if let Ok(Symbol::Symbol(symbol)) = parser.symbol() {
                if symbol_table.contains(&symbol) {
                    continue;
                };

                symbol_table.add_entry(symbol, symbol_counter);
                symbol_counter += 1;
            }
        }
    }
}
