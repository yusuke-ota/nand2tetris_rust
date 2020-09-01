use crate::CommandAssemblyGenerator;
use parser::command_type::CommandType;

impl CommandAssemblyGenerator for CommandType {
    fn as_assembly(&self, filename: &str, segment: String, index: u32) -> Vec<u8> {
        let mut assembly_str = match self {
            CommandType::CArithmetic => unreachable!(),
            CommandType::CPush => c_push(filename, segment, index),
            CommandType::CPop => c_pop(filename, segment, index),
            CommandType::CLabel => c_label(segment, index),
            CommandType::CGoto => c_goto(segment, index),
            CommandType::CIf => c_if(segment, index),
            CommandType::CFunction => c_function(segment, index),
            CommandType::CReturn => c_return(segment, index),
            CommandType::CCall => c_call(segment, index),
        };
        // Assemble_code is utf-8.
        // .as_mut_vec()` is safe when utf-8.
        unsafe { assembly_str.as_mut_vec().clone() }
    }
}

fn c_push(filename: &str, segment: String, index: u32) -> String {
    return match segment.as_str() {
        "constant" => format!(
            "@{1}\n\
                D=A\n\
                {0}",
            WRITE_CURRENT, index
        ),
        "local" | "argument" | "this" | "that" => format!(
            "@{2}\n\
                D=A\n\
                @{1}\n\
                A=M+D\n\
                D=M\n\
                {0}",
            WRITE_CURRENT,
            segment.as_segment(),
            index
        ),
        "pointer" =>
        // pointer address is "3".
        {
            format!(
                "@{1}\n\
                D=M\n\
                {0}",
                WRITE_CURRENT,
                3 + index
            )
        }
        "temp" => format!(
            "@{1}\n\
                D=M\n\
                {0}",
            WRITE_CURRENT,
            5 + index
        ),
        "static" => format!(
            "@{1}.{2}\n\
                D=M\n\
                {0}",
            WRITE_CURRENT, filename, index
        ),
        _ => panic!("c_push(): wrong argument was used."),
    };
}

fn c_pop(filename: &str, segment: String, index: u32) -> String {
    return match segment.as_str() {
        "constant" => unreachable!(),
        "local" | "argument" | "this" | "that" =>
        format!(
            "@{1}\n\
                D=A\n\
                @{0}\n\
                D=M+D\n\
                @R13\n\
                M=D\n\
                @SP\n\
                AM=M-1\n\
                D=M\n\
                @R13\n\
                A=M\n\
                M=D\n",
            segment.as_segment(),
            index
        ),
        "pointer" =>
        // pointer address is "3".
        {
            format!(
                "@SP\n\
                    AM=M-1\n\
                    D=M\n\
                    @{0}\n\
                    M=D\n",
                3 + index
            )
        }
        "temp" => format!(
            "@SP\n\
                AM=M-1\n\
                D=M\n\
                @{0}\n\
                M=D\n",
            5 + index
        ),
        "static" => format!(
            "@SP\n\
                AM=M-1\n\
                D=M\n\
                @{0}.{1}\n\
                M=D\n",
            filename, index
        ),
        _ => panic!("c_push(): wrong argument was used."),
    };
}

fn c_label(segment: String, index: u32) -> String {
    // todo: 8章
    format!("{} {}", segment, index)
}
fn c_goto(segment: String, index: u32) -> String {
    // todo: 8章
    format!("{} {}", segment, index)
}
fn c_if(segment: String, index: u32) -> String {
    // todo: 8章
    format!("{} {}", segment, index)
}
fn c_function(segment: String, index: u32) -> String {
    // todo: 8章
    format!("{} {}", segment, index)
}
fn c_return(segment: String, index: u32) -> String {
    // todo: 8章
    format!("{} {}", segment, index)
}
fn c_call(segment: String, index: u32) -> String {
    // todo: 8章
    format!("{} {}", segment, index)
}

const WRITE_CURRENT: &'static str = "@SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        M=M+1\n";

trait StringUtility {
    fn as_segment(&self) -> &'static str;
}

impl StringUtility for String {
    fn as_segment(&self) -> &'static str {
        match self.as_str() {
            "local" => "LOL",
            "argument" => "ARG",
            "this" => "THIS",
            "that" => "THAT",
            _ => panic!(
                "as_segment(): wrong string. this can translate local, argument, this and that"
            ),
        }
    }
}
