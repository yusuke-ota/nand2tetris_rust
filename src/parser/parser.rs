use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::parser::enums::*;

struct Parser{
    buffer: Vec<String>,
    command: Option<String>
}

impl Parser{
    fn new(mut file: File) -> Self{
        let mut buf_reader = BufReader::new(file);
        let mut buf = String::new();
        let mut buffer = Vec::new();
        while buf_reader.read_line(&mut buf).unwrap_or(0) > 0{
            buffer.push(buf.trim().to_string());
        }

        Self{
            buffer,
            command: Option::None
        }
    }

    fn has_more_commands(&self) -> bool{
        match self.buffer.len(){
            len if len > 0 => true,
            _ => false
        }
    }

    fn advance(&mut self){
        if !self.has_more_commands(){
            return;
        }
        // self.buffer.len() > 0なので、必ず値がある
        let new_command = self.buffer.pop().unwrap();
        self.command = Some(new_command);
    }

    fn command_type(&self) -> CommandType{
        let first_char = self.command.unwrap().chars().next();
        match first_char {
            Some(first_char) if first_char == "@".parse().unwrap() => CommandType::ACommand(self.command.unwrap()),
            Some(first_char) if first_char == "(".parse().unwrap() => CommandType::LCommand(self.command.unwrap()),
            Some(first_char) => CommandType::CCommand(command),
            _ => panic!("Can'not detect command type!")
        }
    }

    fn symbol(&self) -> Result<Symbol, Result::Err>{
        let command_type = self.command_type();
        match command_type {
            CommandType::ACommand(command) => return Ok(classification_symbol(command)),
            CommandType::LCommand(command) => return Ok(classification_symbol(command)),
            _ => Err("This type is not ACommand or LCommand!")
        }
    }

    fn dest(&self) -> Result<DestType, Result::Err>{
        let mut c_command = String::with_capacity(16);
        match self.command_type() {
            CommandType::CCommand(command) => c_command = command,
            _ => return Err("This type is not CCommand!")
        }
        let c_command = c_command.chars().collect::<Vec<char>>();
        let dest = c_command[10..13].iter().map(|&iter| iter as i32 - 48).collect::<[i32;3]>();
        return match dest {
            dest if dest == [0, 0, 0] => Ok(DestType::Null),
            dest if dest == [0, 0, 1] => Ok(DestType::M),
            dest if dest == [0, 1, 0] => Ok(DestType::D),
            dest if dest == [0, 1, 1] => Ok(DestType::MD),
            dest if dest == [1, 0, 0] => Ok(DestType::A),
            dest if dest == [1, 0, 1] => Ok(DestType::AM),
            dest if dest == [1, 1, 0] => Ok(DestType::AD),
            dest if dest == [1, 1, 1] => Ok(DestType::AMD),
            _ => Err("Dest parse error!")
        }
    }

    fn comp(&self) -> Result<CompType, Result::Err>{
        let mut c_command = String::with_capacity(16);
        match self.command_type() {
            CommandType::CCommand(command) => c_command = command,
            _ => return Err("This type is not CCommand!")
        }
        let c_command = c_command.chars().collect::<Vec<char>>();
        let comp = c_command[3..10].iter().map(|&iter| iter as i32 - 48).collect::<[i32;7]>();
        return match comp {
            comp if [0, 0, 0, 0, 0, 0, 0] => CompType
        }

    }

    fn jump(&self) -> Result<JumpType, Result::Err>{
        let mut c_command = String::with_capacity(16);
        match self.command_type() {
            CommandType::CCommand(command) => c_command = command,
            _ => return Err("This type is not CCommand!")
        }
        let c_command = c_command.chars().collect::<Vec<char>>();
        let jump = c_command[13..=16].iter().map(|&iter| iter as i32 - 48).collect::<[i32;3]>();
        return match jump {
            jump if jump == [0, 0, 0] => Ok(JumpType::Null),
            jump if jump == [0, 0, 1] => Ok(JumpType::JGT),
            jump if jump == [0, 1, 0] => Ok(JumpType::JEQ),
            jump if jump == [0, 1, 1] => Ok(JumpType::JGE),
            jump if jump == [1, 0, 0] => Ok(JumpType::JLT),
            jump if jump == [1, 0, 1] => Ok(JumpType::JNE),
            jump if jump == [1, 1, 0] => Ok(JumpType::JLE),
            jump if jump == [1, 1, 1] => Ok(JumpType::JMP),
            _ => Err("Jump parse error!")
        }
    }
}