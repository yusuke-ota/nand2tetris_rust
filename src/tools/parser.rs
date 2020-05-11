use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::tools::*;

#[derive(Clone)]
pub struct Parser {
    buffer: Vec<String>,
    command: Option<String>,
}

impl Parser{
    pub fn new(file: File) -> Self{
        let mut buf_reader = BufReader::new(file);
        let mut buf = String::new();
        let mut buffer = Vec::new();
        while buf_reader.read_line(&mut buf).unwrap_or(0) > 0{
            let trim_buf = buf.trim_end();
            if !trim_buf.starts_with('/') && trim_buf != "" {
            buffer.push(trim_buf.to_string());
            }
            buf.clear();
        }

        Self{
            buffer,
            command: Option::None
        }
    }

    pub fn has_more_commands(&self) -> bool{
        match self.buffer.len(){
            len if len > 0 => true,
            _ => false
        }
    }

    pub fn advance(&mut self){
        if !self.has_more_commands(){
            return;
        }
        // self.buffer.len() > 0なので、必ず値がある
        let new_command = self.buffer.pop().unwrap();
        self.command = Some(new_command);
    }

    pub fn command_type(&self) -> CommandType{
        let first_char = self.command.as_ref().unwrap().chars().next().unwrap();
        return match first_char {
            '@' => CommandType::ACommand(self.command.clone().unwrap()),
            '(' => CommandType::LCommand(self.command.clone().unwrap()),
            'A' | 'D' | 'M' => CommandType::CCommand(self.command.clone().unwrap()),
            _ => panic!("Can'not detect command type!")
        }
    }

    pub fn symbol(&self) -> Result<Symbol, &'static str>{
        let command_type = self.command_type();
        return match command_type {
            CommandType::ACommand(command) => {
                let num = command.trim_start_matches('@');
                Ok(classification_symbol(num))
            },
            CommandType::LCommand(command) => {
                let symbol = command.trim_start_matches('(').trim_end_matches(')');
                Ok(classification_symbol(symbol))
            },
            _ => Err("This type is not ACommand or LCommand!")
        }
    }

    pub fn dest(&self) -> Result<DestType, &'static str>{
        let c_command;
        match self.command_type() {
            CommandType::CCommand(command) => c_command = command,
            _ => return Err("This type is not CCommand!")
        }
        let mut tmp = [0; 4];
        let separate_place = c_command.find(|char: char| char.encode_utf8(&mut tmp) == "=" || char.encode_utf8(&mut tmp) == ";");
        let dest_string;
        match separate_place {
            Some(num) => dest_string = c_command[0..num].to_string(),
            // Some(num) => dest_string = {
            //     let mut chars = c_command.chars();
            //     let mut counter:usize = 0;
            //     let mut dest_string = String::with_capacity(num);
            //     while counter <= num {
            //         dest_string.push(chars.nth(0).unwrap());
            //         counter += 1;
            //     };
            //     dest_string
            // },
            None => return Err("Cannot found '=' or ';'")
        }
        let dest_string: &str = &dest_string;
        return match dest_string {
            "null" => Ok(DestType::Null),
            "M" => Ok(DestType::M),
            "D" => Ok(DestType::D),
            "MD" => Ok(DestType::MD),
            "A" => Ok(DestType::A),
            "AM" => Ok(DestType::AM),
            "AD" => Ok(DestType::AD),
            "AMD" => Ok(DestType::AMD),
            _ => Err("Cannot parse to DestType")
        }
    }

    pub fn comp(&self) -> Result<CompType, &'static str>{
        let c_command;
        match self.command_type() {
            CommandType::CCommand(command) => c_command = command,
            _ => return Err("This type is not CCommand!")
        }
        let separate_place = c_command.find("=");
        let comp_string;
        match separate_place {
            Some(num) => comp_string = c_command[num..].to_string(),
            None => return Err("Cannot found '='")
        }
        let comp_string: &str = &comp_string;
        return match comp_string {
            "0" => Ok(CompType::Zero),
            "1" => Ok(CompType::One),
            "-1" => Ok(CompType::MMinusOne),
            "D" => Ok(CompType::D),
            "A" => Ok(CompType::A),
            "!D" => Ok(CompType::NotD),
            "!A" => Ok(CompType::NotA),
            "-D" => Ok(CompType::MinusD),
            "-A" => Ok(CompType::MinusA),
            "D+1" => Ok(CompType::DPlusOne),
            "A+1" => Ok(CompType::APlusOne),
            "D-1" => Ok(CompType::DMinusOne),
            "A-1" => Ok(CompType::AMinusOne),
            "D+A" => Ok(CompType::DPlusA),
            "D-A" => Ok(CompType::DMinusA),
            "A-D" => Ok(CompType::AMinusD),
            "D&A" => Ok(CompType::DAndA),
            "D||A" => Ok(CompType::DOrA),
            "M" => Ok(CompType::M),
            "!M" => Ok(CompType::NotM),
            "-M" => Ok(CompType::MinusM),
            "M+1" => Ok(CompType::MPlusOne),
            "M-1" => Ok(CompType::MMinusOne),
            "D+M" => Ok(CompType::DPlusM),
            "D-M" => Ok(CompType::DMinusM),
            "M-D" => Ok(CompType::MMinusD),
            "D&M" => Ok(CompType::DAndM),
            "D||M" => Ok(CompType::DOrM),
            _ => Err("Unexpected value of Comp")
        }
    }

    pub fn jump(&self) -> Result<JumpType, &'static str>{
        let c_command;
        match self.command_type() {
            CommandType::CCommand(command) => c_command = command,
            _ => return Err("This type is not CCommand!")
        }
        let separate_place = c_command.find(";");
        let jump_string;
        match separate_place {
            Some(num) => jump_string = c_command[num..].to_string(),
            None => return Err("Cannot found ';'")
        }
        let jump_string: &str = &jump_string;
        return match jump_string {
            "null" => Ok(JumpType::Null),
            "JGT" => Ok(JumpType::JGT),
            "JEQ" => Ok(JumpType::JEQ),
            "JGE" => Ok(JumpType::JGE),
            "JLT" => Ok(JumpType::JLT),
            "JNE" => Ok(JumpType::JNE),
            "JLE" => Ok(JumpType::JLE),
            "JMP" => Ok(JumpType::JMP),
            _ => Err("Jump parse error!")
        }
    }
}