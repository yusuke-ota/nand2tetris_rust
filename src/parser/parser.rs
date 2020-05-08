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
            [0, 0, 0] => Ok(DestType::Null),
            [0, 0, 1] => Ok(DestType::M),
            [0, 1, 0] => Ok(DestType::D),
            [0, 1, 1] => Ok(DestType::MD),
            [1, 0, 0] => Ok(DestType::A),
            [1, 0, 1] => Ok(DestType::AM),
            [1, 1, 0] => Ok(DestType::AD),
            [1, 1, 1] => Ok(DestType::AMD),
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
            [0, 1, 0, 1, 0, 1, 0] => Ok(CompType::Zero),
            [0, 1, 1, 1, 1, 1, 1] => Ok(CompType::One),
            [0, 1, 1, 1, 0, 1, 0] => Ok(CompType::MinusOne),
            [0, 0, 0, 1, 1, 0, 0] => Ok(CompType::D),
            [0, 1, 1, 0, 0, 0, 0] => Ok(CompType::A),
            [0, 0, 0, 1, 1, 0, 1] => Ok(CompType::NotD),
            [0, 1, 1, 0, 0, 0, 1] => Ok(CompType::NotA),
            [0, 0, 0, 1, 1, 1, 1] => Ok(CompType::MinusD),
            [0, 1, 1, 0, 0, 1, 1] => Ok(CompType::MinusA),
            [0, 0, 1, 1, 1, 1, 1] => Ok(CompType::DPlusOne),
            [0, 1, 1, 0, 1, 1, 1] => Ok(CompType::APlusOne),
            [0, 0, 0, 1, 1, 1, 0] => Ok(CompType::DMinusOne),
            [0, 1, 1, 0, 0, 1, 0] => Ok(CompType::AMinusOne),
            [0, 0, 0, 0, 0, 1, 0] => Ok(CompType::DPlusA),
            [0, 1, 0, 0, 0, 1, 1] => Ok(CompType::DMinusA),
            [0, 0, 0, 0, 1, 1, 1] => Ok(CompType::AMinusD),
            [0, 0, 0, 0, 0, 0, 0] => Ok(CompType::DAndA),
            [0, 0, 1, 0, 1, 0, 1] => Ok(CompType::DOrA),
            [1, 1, 1, 0, 0, 0, 0] => Ok(CompType::M),
            [1, 1, 1, 0, 0, 0, 1] => Ok(CompType::NotM),
            [1, 1, 1, 0, 0, 1, 1] => Ok(CompType::MinusM),
            [1, 1, 1, 0, 1, 1, 1] => Ok(CompType::MPlusOne),
            [1, 1, 1, 0, 0, 1, 0] => Ok(CompType::MMinusOne),
            [1, 0, 0, 0, 0, 1, 0] => Ok(CompType::DPlusM),
            [1, 1, 0, 0, 0, 1, 1] => Ok(CompType::DMinusM),
            [1, 0, 0, 0, 1, 1, 1] => Ok(CompType::MMinusD),
            [1, 0, 0, 0, 0, 0, 0] => Ok(CompType::DAndM),
            [1, 0, 1, 0, 1, 0, 1] => Ok(CompType::DOrM),
            _ => Err("Unexpected value of Comp")
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
            [0, 0, 0] => Ok(JumpType::Null),
            [0, 0, 1] => Ok(JumpType::JGT),
            [0, 1, 0] => Ok(JumpType::JEQ),
            [0, 1, 1] => Ok(JumpType::JGE),
            [1, 0, 0] => Ok(JumpType::JLT),
            [1, 0, 1] => Ok(JumpType::JNE),
            [1, 1, 0] => Ok(JumpType::JLE),
            [1, 1, 1] => Ok(JumpType::JMP),
            _ => Err("Jump parse error!")
        }
    }
}