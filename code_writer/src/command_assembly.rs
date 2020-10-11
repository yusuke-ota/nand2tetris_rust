use crate::CommandAsAssembly;
use parser::command_type::CommandType;

impl CommandAsAssembly for CommandType {
    fn as_assembly(&self, filename: &str, segment: String, index: u32) -> Vec<u8> {
        let mut assembly_str = match self {
            CommandType::CArithmetic => unreachable!(),
            CommandType::CPush => c_push(filename, segment, index),
            CommandType::CPop => c_pop(filename, segment, index),
            CommandType::CLabel => c_label(segment),
            CommandType::CGoto => c_goto(segment),
            CommandType::CIf => c_if(segment),
            CommandType::CFunction => c_function(segment, index),
            CommandType::CReturn => c_return(),
            CommandType::CCall => c_call(segment, index),
        };
        // Assemble_code is utf-8.
        // .as_mut_vec()` is safe when utf-8.
        unsafe { assembly_str.as_mut_vec().clone() }
    }
}

fn c_push(filename: &str, segment: String, index: u32) -> String {
    match segment.as_str() {
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
    }
}

fn c_pop(filename: &str, segment: String, index: u32) -> String {
    match segment.as_str() {
        "constant" => unreachable!(),
        "local" | "argument" | "this" | "that" => format!(
            "@{0}\n\
                D=M\n\
                @{1}\n\
                D=D+A\n\
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
    }
}

fn c_label(segment: String) -> String {
    format!("({})\n", segment)
}

fn c_goto(segment: String) -> String {
    format!(
        "@{}\n\
        0;JMP\n",
        segment)
}

fn c_if(segment: String) -> String {
    format!(
        "@SP\n\
        AM=M-1\n\
        D=M\n\
        @{}\n\
        D;JNE\n",
        segment)
}

fn c_function(function_name: String, num_locals: u32) -> String {
    // memory allocate before process.
    // (function_name).len()\n = function_name.len() + 3
    let mut function_assembly = String::with_capacity(function_name.len() + 3 + 23 * num_locals as usize);

    function_assembly.push_str("(");
    function_assembly.push_str(function_name.as_str());
    function_assembly.push_str(")\n");
    for _ in 0..num_locals {
        // One loop is 23 byte string.
        function_assembly.push_str(WRITE_CURRENT_ZERO)
    }
    function_assembly
}

fn c_return() -> String {
    return format!("{}{}{}{}{}{}{}{}{}",
            SET_LCL_TO_R14,
            SET_RETURN_ADDRESS_TO_R15,
            POP_TO_ARG,
            UPDATE_ARG_TO_SP,
            return_caller_value("THAT"),
            return_caller_value("THIS"),
            return_caller_value("ARG"),
            return_caller_value("LCL"),
            GOTO_R15
    );

    const SET_LCL_TO_R14: &str =
        "@LCL\n\
        D=M\n\
        @R14\n\
        M=D\n";
    const SET_RETURN_ADDRESS_TO_R15: &str =
        "@5\n\
        A=D-A\n\
        D=M\n\
        @R15\n\
        M=D\n";
    const POP_TO_ARG: &str =
        "@SP\n\
        AM=M-1\n\
        D=M\n\
        @ARG\n\
        A=M\n\
        M=D\n";
    const UPDATE_ARG_TO_SP: &str =
        "@ARG\n\
        D=M\n\
        @SP\n\
        M=D+1\n";
    fn return_caller_value(segment: &str) -> String{
        format!(
        "@R14\n\
        AM=M-1\n\
        D=M\n\
        @{}\n\
        M=D\n",
        segment)
    }
    const GOTO_R15: &str =
        "@R15\n\
        A=M\n\
        0;JMP\n";
}

fn c_call(function_name: String, arg_num: u32) -> String {
    // todo: 8章
    unimplemented!();
    format!("{} {}", function_name, arg_num)
}

const WRITE_CURRENT: &str =
    "@SP\n\
    A=M\n\
    M=D\n\
    @SP\n\
    M=M+1\n";

const WRITE_CURRENT_ZERO: &str =
    "@SP\n\
    A=M\n\
    M=0\n\
    @SP\n\
    AM=M+1\n";

trait StringUtility {
    fn as_segment(&self) -> &'static str;
}

impl StringUtility for String {
    fn as_segment(&self) -> &'static str {
        match self.as_str() {
            "local" => "LCL",
            "argument" => "ARG",
            "this" => "THIS",
            "that" => "THAT",
            _ => panic!(
                "as_segment(): wrong string. this can translate local, argument, this and that"
            ),
        }
    }
}
