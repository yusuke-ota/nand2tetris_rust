use std::convert::TryFrom;

pub enum ArithmeticType {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl TryFrom<&str> for ArithmeticType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "add" => Ok(Self::Add),
            "sub" => Ok(Self::Sub),
            "neg" => Ok(Self::Neg),
            "eq" => Ok(Self::Eq),
            "gt" => Ok(Self::Gt),
            "lt" => Ok(Self::Lt),
            "and" => Ok(Self::And),
            "or" => Ok(Self::Or),
            "not" => Ok(Self::Not),
            _ => Err("Convert error"),
        }
    }
}

// todo: code_writerに移動
impl ArithmeticType {
    pub fn as_assembly(&self, label_num: &mut u32) -> Vec<u8> {
        return match self {
            ArithmeticType::Add => ADD.to_vec(),
            ArithmeticType::Sub => SUB.to_vec(),
            ArithmeticType::Neg => NEG.to_vec(),
            ArithmeticType::Eq => eq(label_num),
            ArithmeticType::Gt => gt(label_num),
            ArithmeticType::Lt => lt(label_num),
            ArithmeticType::And => AND.to_vec(),
            ArithmeticType::Or => OR.to_vec(),
            ArithmeticType::Not => NOT.to_vec(),
        };
    }
}

const ADD: &'static [u8; 42] =
    // スタックポインタの一番上のアドレス＋1が入っているアドレスを参照、Aに代入(Rust的に言うとA=&SP, M=SP)
    // M=M-1(=一番上のアドレス)して、そのアドレスをAに代入
    // DレジスタにM(=A(一番上のアドレス)の中身)を代入
    // スタックポインタの一番上のアドレス＋1が入っているアドレスを参照、Aに代入
    // DレジスタにM(=A(一番上のアドレス)の中身)を代入
    // スタックの一番上に現在のDレジスタの値-Aレジスタの値を代入する
    // スタックを一つ進める
    b"@SP\n\
    AM=M-1\n\
    D=M\n\
    @SP\n\
    AM=M-1\n\
    M=D+M\n\
    @SP\n\
    M=M+1\n";
const SUB: &'static [u8; 42] =
    // A-Bがあったとき、M=D-MだとB-Aになってしまう
    // 順番注意
    b"@SP\n\
    AM=M-1\n\
    D=A\n\
    @SP\n\
    AM=M-1\n\
    M=M-D\n\
    @SP\n\
    M=M+1\n";
const NEG: &'static [u8; 26] = b"@SP\n\
    AM=M-1\n\
    M=-M\n\
    @SP\n\
    M=M+1\n";
const AND: &'static [u8; 42] = b"@SP\n\
    AM=M-1\n\
    D=A\n\
    @SP\n\
    AM=M-1\n\
    M=D&M\n\
    @SP\n\
    M=M+1\n";
const OR: &'static [u8; 42] = b"@SP\n\
    AM=M-1\n\
    D=A\n\
    @SP\n\
    AM=M-1\n\
    M=D|M\n\
    @SP\n\
    M=M+1\n";
const NOT: &'static [u8; 26] = b"@SP\n\
    AM=M-1\n\
    M=!M\n\
    @SP\n\
    M=M+1\n";

fn eq(label_num: &mut u32) -> Vec<u8> {
    let temp_num = *label_num + 1;
    let mut assemble_code = format!(
        "@SP\n\
        AM=M-1\n\
        D=A\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @label{0}\n\
        D;JEQ\n\
        M=0\n\
        @label{1}\n\
        (label{0})\n\
        M=-1\n\
        (label{1})\n\
        @SP\n\
        M=M+1\n",
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
        D=A\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @label{0}\n\
        D;JGT\n\
        M=0\n\
        @label{1}\n\
        (label{0})\n\
        M=-1\n\
        (label{1})\n\
        @SP\n\
        M=M+1\n",
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
        D=A\n\
        @SP\n\
        AM=M-1\n\
        D=M-D\n\
        @label{0}\n\
        D;JLT\n\
        M=0\n\
        @label{1}\n\
        (label{0})\n\
        M=-1\n\
        (label{1})\n\
        @SP\n\
        M=M+1\n",
        temp_num,
        temp_num + 1
    );
    *label_num += 2;
    // Assemble_code is utf-8.
    // `.as_mut_vec()` is safe when utf-8.
    unsafe { assemble_code.as_mut_vec().clone() }
}
