use crate::ArithmeticAsAssembly;
use parser::arithmetic_type::ArithmeticType;

impl ArithmeticAsAssembly for ArithmeticType {
    fn as_assembly(&self, label_num: &mut u32) -> Vec<u8> {
        match self {
            ArithmeticType::Add => ADD.to_vec(),
            ArithmeticType::Sub => SUB.to_vec(),
            ArithmeticType::Neg => NEG.to_vec(),
            ArithmeticType::Eq => eq(label_num),
            ArithmeticType::Gt => gt(label_num),
            ArithmeticType::Lt => lt(label_num),
            ArithmeticType::And => AND.to_vec(),
            ArithmeticType::Or => OR.to_vec(),
            ArithmeticType::Not => NOT.to_vec(),
        }
    }
}

// スタックポインタの一番上のアドレス＋1が入っているアドレスを参照、Aに代入(Rust的に言うとA=&SP, M=SP)
// M=M-1(=一番上のアドレス)して、そのアドレスをAに代入
// DレジスタにM(=A(一番上のアドレス)の中身)を代入
// スタックポインタの一番上のアドレス＋1が入っているアドレスを参照、Aに代入
// DレジスタにM(=A(一番上のアドレス)の中身)を代入
// スタックの一番上に現在のDレジスタの値-Aレジスタの値を代入する
// スタックを一つ進める
const ADD: &[u8; 42] = b"@SP\n\
    AM=M-1\n\
    D=M\n\
    @SP\n\
    AM=M-1\n\
    M=D+M\n\
    @SP\n\
    M=M+1\n";

// A-Bがあったとき、M=D-MだとB-Aになってしまう
// 順番注意
const SUB: &[u8; 42] = b"@SP\n\
    AM=M-1\n\
    D=M\n\
    @SP\n\
    AM=M-1\n\
    M=M-D\n\
    @SP\n\
    M=M+1\n";

const NEG: &[u8; 26] = b"@SP\n\
    AM=M-1\n\
    M=-M\n\
    @SP\n\
    M=M+1\n";

const AND: &[u8; 42] = b"@SP\n\
    AM=M-1\n\
    D=M\n\
    @SP\n\
    AM=M-1\n\
    M=D&M\n\
    @SP\n\
    M=M+1\n";

const OR: &[u8; 42] = b"@SP\n\
    AM=M-1\n\
    D=M\n\
    @SP\n\
    AM=M-1\n\
    M=D|M\n\
    @SP\n\
    M=M+1\n";

const NOT: &[u8; 26] = b"@SP\n\
    AM=M-1\n\
    M=!M\n\
    @SP\n\
    M=M+1\n";

fn eq(label_num: &mut u32) -> Vec<u8> {
    let temp_num = *label_num + 1;
    let mut assemble_code = format!(
        "@SP\n\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @label{0}\n\
        D;JEQ\n\
        D=0\n\
        @label{1}\n\
        0;JMP\n\
        (label{0})\n\
        D=-1\n\
        (label{1})\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        AM=M+1\n",
        temp_num,
        temp_num + 1
    );
    *label_num += 2;
    // Assemble_code is utf-8.
    // `.as_mut_vec()` is safe when utf-8.
    unsafe { assemble_code.as_mut_vec().clone() }
}

fn gt(label_num: &mut u32) -> Vec<u8> {
    let temp_num = *label_num + 1;
    let mut assemble_code = format!(
        "@SP\n\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @label{0}\n\
        D;JGT\n\
        D=0\n\
        @label{1}\n\
        0;JMP\n\
        (label{0})\n\
        D=-1\n\
        (label{1})\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        AM=M+1\n",
        temp_num,
        temp_num + 1
    );
    *label_num += 2;
    // Assemble_code is utf-8.
    // `.as_mut_vec()` is safe when utf-8.
    unsafe { assemble_code.as_mut_vec().clone() }
}

fn lt(label_num: &mut u32) -> Vec<u8> {
    let temp_num = *label_num + 1;
    let mut assemble_code = format!(
        "@SP\n\
        AM=M-1\n\
        D=M\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @label{0}\n\
        D;JLT\n\
        D=0\n\
        @label{1}\n\
        0;JMP\n\
        (label{0})\n\
        D=-1\n\
        (label{1})\n\
        @SP\n\
        A=M\n\
        M=D\n\
        @SP\n\
        AM=M+1\n",
        temp_num,
        temp_num + 1
    );
    *label_num += 2;
    // Assemble_code is utf-8.
    // `.as_mut_vec()` is safe when utf-8.
    unsafe { assemble_code.as_mut_vec().clone() }
}
