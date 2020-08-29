use crate::tools::{classification_symbol, CommandType, CompType, DestType, JumpType, Symbol};
use crate::{IParser, Parser};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

impl IParser for Parser {
    fn new(file: File) -> Self {
        let mut buf_reader = BufReader::new(file);
        let mut buf = String::new();
        let mut buffer = Vec::new();
        while buf_reader.read_line(&mut buf).unwrap_or(0) > 0 {
            let trimmed_buf = buf.trim_end().trim_start();
            if !trimmed_buf.starts_with('/') && trimmed_buf != "" {
                buffer.push(trimmed_buf.to_string());
            }
            buf.clear();
        }
        // pop以外のメソッドで、配列から順序を崩さずに値を取り出すことができない
        // なので、reverse()して、配列の最後尾(実質頭)から値を取り出す
        // In Rust, we can't use shift command to collection type like Ruby language.
        // So, use reverse() and pop() to pick the first element of Vec<T>.
        buffer.reverse();
        Self {
            buffer,
            command: Option::None,
        }
    }

    fn has_more_commands(&self) -> bool {
        match self.buffer.len() {
            len if len > 0 => true,
            _ => false,
        }
    }

    fn advance(&mut self) {
        if !self.has_more_commands() {
            panic!();
        }
        // self.buffer.len() > 0なので、必ず値がある
        // This "unwrap()" is always success because "self.buffer.len() > 0".
        let new_command = self.buffer.pop().unwrap();
        self.command = Some(new_command);
    }

    fn command_type(&self) -> Result<CommandType, &'static str> {
        let first_char = self.command.as_ref().unwrap().chars().next().unwrap();
        return match first_char {
            '@' => Ok(CommandType::ACommand(self.command.clone().unwrap())),
            '(' => Ok(CommandType::LCommand(self.command.clone().unwrap())),
            'A' | 'D' | 'M' | '0' => Ok(CommandType::CCommand(self.command.clone().unwrap())),
            _ => Err("Can't detect command type!"),
        };
    }

    fn symbol(&self) -> Result<Symbol, &'static str> {
        return match self.command_type() {
            // A command: "@xxx" (x is number)
            Ok(CommandType::ACommand(command)) => {
                let num = command.trim_start_matches('@');
                Ok(classification_symbol(num))
            }
            // L command: "(xxx)" (x is string)
            Ok(CommandType::LCommand(command)) => {
                let symbol = command.trim_start_matches('(').trim_end_matches(')');
                Ok(classification_symbol(symbol))
            }
            _ => Err("This type is not ACommand or LCommand!"),
        };
    }

    fn dest(&self) -> Result<DestType, &'static str> {
        let c_command;
        match self.command_type() {
            Ok(CommandType::CCommand(command)) => c_command = command,
            _ => return Err("This type is not CCommand!"),
        }
        let separate_place = c_command.find('=');
        let dest_string;
        match separate_place {
            Some(num) => dest_string = c_command[0..num].to_string(),
            None => return Err("Cannot found ="),
        }
        DestType::try_from(dest_string.as_str())
    }

    fn comp(&self) -> Result<CompType, &'static str> {
        let c_command;
        match self.command_type() {
            Ok(CommandType::CCommand(command)) => c_command = command,
            _ => return Err("This type is not CCommand!"),
        }

        let space_and_comment: &[_] = &['/', ' '];
        let white_space_or_comment = c_command.find(space_and_comment);
        let end = white_space_or_comment.unwrap_or(c_command.len());

        let comp_string;
        // c command: "x=xxx" or "x;xxx" ()
        let separate_place_equal = c_command.find('=');
        let separate_place_semi_colon = c_command.find(';');

        if let Some(separate) = separate_place_equal {
            comp_string = c_command[separate + 1..end].to_string();
        } else if let Some(separate) = separate_place_semi_colon {
            comp_string = c_command[0..separate].to_string();
        } else {
            return Err("Cannot found = or ;");
        }
        CompType::try_from(comp_string.as_str())
    }

    fn jump(&self) -> Result<JumpType, &'static str> {
        let c_command;
        match self.command_type() {
            Ok(CommandType::CCommand(command)) => c_command = command,
            _ => return Err("This type is not CCommand!"),
        }
        let space_and_comment: &[_] = &['/', ' '];
        let white_space_or_comment = c_command.find(space_and_comment);
        let end = white_space_or_comment.unwrap_or(c_command.len());

        let separate_place = c_command.find(";");
        let jump_string;
        match separate_place {
            Some(num) => jump_string = c_command[num + 1..end].to_string(),
            None => return Err("Cannot found ;"),
        }
        JumpType::try_from(jump_string.as_str())
    }
}
