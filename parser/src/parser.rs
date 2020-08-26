use crate::command_type::CommandType;
use crate::{IParser, Parser};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Parser {
    pub fn new(file: File) -> Self {
        let buf_reader = BufReader::new(file);
        let mut stream = separate_line(buf_reader);
        // To get stream head with `.pop()`, stream should `.reverse()`.
        // `.pop()` get "last" element of argument.
        stream.reverse();

        Self {
            stream,
            command: None,
        }
    }
}

impl IParser for Parser {
    fn has_more_commands(&self) -> bool {
        self.stream.len() > 0
    }

    fn advance(&mut self) {
        match self.has_more_commands() {
            true => {
                let command = self.stream.pop();
                self.command = command;
            }
            false => (),
        }
    }

    /// Panic when command == None, command == "x", and "non_command _ _"
    fn command_type(&self) -> CommandType {
        let command = self.command.clone().unwrap().clone();
        let command = command
            .split_whitespace()
            .next()
            .expect("command_type(): String == \"\"");
        CommandType::try_from(command).unwrap()
    }

    fn arg1(&self) -> String {
        let command = self.command.clone().unwrap().clone();
        let command = command.split_whitespace().collect::<Vec<&str>>();
        return match command[..] {
            ["add"] => "add".to_string(),
            ["sub"] => "sub".to_string(),
            ["neg"] => "neg".to_string(),
            ["eq"] => "eq".to_string(),
            ["gt"] => "gt".to_string(),
            ["lt"] => "lt".to_string(),
            ["and"] => "and".to_string(),
            ["or"] => "or".to_string(),
            ["not"] => "not".to_string(),
            [_command, arg1, ..] => arg1.to_string(),
            _ => panic!("arg1(): unexpected argument."),
        };
    }

    fn arg2(&self) -> i32 {
        let command = self.command.clone().unwrap().clone();
        let command = command.split_whitespace().collect::<Vec<&str>>();
        command[2].parse::<i32>().expect("arg2(): parse error.")
    }
}

fn separate_line(mut buf_reader: BufReader<File>) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let mut buf = String::new();

    // When read_line() return err, all lines in buf_reader are processed.
    while buf_reader.read_line(&mut buf).unwrap_or(0) > 0 {
        let trimmed_buf = buf.trim_end().trim_start();
        if !trimmed_buf.starts_with('/') && trimmed_buf != "" {
            result.push(trimmed_buf.to_string());
        }
        buf.clear();
    }

    result
}
