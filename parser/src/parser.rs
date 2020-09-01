use crate::command_type::CommandType;
use crate::{Parser, ParserPublicAPI};
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

impl ParserPublicAPI for Parser {
    fn has_more_commands(&self) -> bool {
        self.stream.len() > 0
    }

    fn advance(&mut self) {
        let command = self.stream.pop();
        self.command = command;
    }

    /// Panic when command == None, command == "x", and "non_command _ _"
    fn command_type(&self) -> CommandType {
        let command = self.command.as_ref().unwrap().clone();
        let command = command
            .split_whitespace()
            .next()
            .expect("command_type(): String == \"\"");
        CommandType::try_from(command).expect("convert failed")
    }

    fn arg1(&self) -> String {
        let command = self.command.as_ref().unwrap().clone();
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

    fn arg2(&self) -> u32 {
        let command = self.command.clone().unwrap().clone();
        let command = command.split_whitespace().collect::<Vec<&str>>();
        command[2].parse::<u32>().expect("arg2(): parse error.")
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

#[cfg(test)]
mod tests {
    use crate::command_type::CommandType;
    use crate::{Parser, ParserPublicAPI};

    fn generate_dummy_parser(stream: &str) -> Parser {
        let mut stream: Vec<String> = stream.split("\n").map(|str| str.to_string()).collect();
        stream.reverse();
        Parser {
            stream,
            command: None,
        }
    }

    #[test]
    fn has_more_commands_test() {
        let mut dummy_parser = generate_dummy_parser("first line");
        assert_eq!(dummy_parser.has_more_commands(), true);
        dummy_parser.stream.pop();
        assert_eq!(dummy_parser.has_more_commands(), false);
    }
    #[test]
    fn advance_test() {
        let mut dummy_parser = generate_dummy_parser("first line");
        dummy_parser.advance();
        assert_eq!(dummy_parser.command, Some("first line".to_string()));
        dummy_parser.advance();
        assert_eq!(dummy_parser.command, None);
    }

    #[test]
    fn command_type_test() {
        let mut dummy_parser = generate_dummy_parser(
            "add\nsub\nneg\neq\ngt\nlt\nand\nor\nnot\n\
            push\npop\n",
        );
        let compare_list = [
            CommandType::CArithmetic,
            CommandType::CPush,
            CommandType::CPop,
        ];
        for _ in 0..9_usize {
            dummy_parser.advance();
            assert_eq!(dummy_parser.command_type(), compare_list[0]);
        }
        for index in 9..11_usize {
            dummy_parser.advance();
            assert_eq!(dummy_parser.command_type(), compare_list[index - 8]);
        }
    }

    #[test]
    fn arg1_test() {
        let mut dummy_parser = generate_dummy_parser(
            "add\nsub\nneg\neq\ngt\nlt\nand\nor\nnot\n\
            push local 2\npop local 2\n", // todo: 8章
        );
        let compare_list = [
            "add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not", "local",
            "local", // todo: 8章
        ];
        for index in 0..11_usize {
            dummy_parser.advance();
            assert_eq!(dummy_parser.arg1(), compare_list[index]);
        }
    }

    #[test]
    fn arg2_test() {
        let mut dummy_parser = generate_dummy_parser(
            "push local 1\npop local 2\n", // todo: 8章
        );
        let compare_list = [1, 2, 3, 4];
        for index in 0..2_usize {
            dummy_parser.advance();
            assert_eq!(dummy_parser.arg2(), compare_list[index]);
        }
    }
}
