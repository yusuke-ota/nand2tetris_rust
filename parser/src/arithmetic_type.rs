use std::convert::TryFrom;

pub enum ArithmeticType{
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

impl TryFrom<&str> for ArithmeticType{
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
            _ => Err("Convert error")
        }
    }
}

impl From<ArithmeticType> for &'static str{
    fn from(arithmetic_type: ArithmeticType) -> Self {
        return match arithmetic_type {
            ArithmeticType::Add => ADD,
            ArithmeticType::Sub => SUB,
            ArithmeticType::Neg => NEG,
            ArithmeticType::Eq => eq(),
            ArithmeticType::Gt => gt(),
            ArithmeticType::Lt => lt(),
            ArithmeticType::And => AND,
            ArithmeticType::Or => OR,
            ArithmeticType::Not => NOT,
        }
    }
}

const ADD: &'static str =
    // スタックポインタの一番上のアドレス＋1が入っているアドレスを参照、Aに代入(Rust的に言うとA=&SP, M=SP)
    // M=M-1(=一番上のアドレス)して、そのアドレスをAに代入
    // DレジスタにM(=A(一番上のアドレス)の中身)を代入
    // スタックポインタの一番上のアドレス＋1が入っているアドレスを参照、Aに代入
    // DレジスタにM(=A(一番上のアドレス)の中身)を代入
    // スタックの一番上に現在のDレジスタの値-Aレジスタの値を代入する
    // スタックを一つ進める
    "@SP\n\
    AM=M-1\n\
    D=M\n\
    @SP\n\
    AM=M-1\n\
    M=D+M\n\
    @SP\n\
    M=M+1\n";
const SUB: &'static str =
    // A-Bがあったとき、M=D-MだとB-Aになってしまう
    // 順番注意
    "@SP\n\
    AM=M-1\n\
    D=A\n\
    @SP\n\
    AM=M-1\n\
    M=M-D\n\
    @SP\n\
    M=M+1\n";
const NEG: &'static str =
    "@SP\n\
    AM=M-1\n\
    M=-M\n\
    @SP\n\
    M=M+1\n";
const AND: &'static str =
    "@SP\n\
    AM=M-1\n\
    D=A\n\
    @SP\n\
    AM=M-1\n\
    M=D&M\n\
    @SP\n\
    M=M+1\n";
const OR: &'static str =
    "@SP\n\
    AM=M-1\n\
    D=A\n\
    @SP\n\
    AM=M-1\n\
    M=D|M\n\
    @SP\n\
    M=M+1\n";
const NOT: &'static str =
    "@SP\n\
    AM=M-1\n\
    M=!M\n\
    @SP\n\
    M=M+1\n";
// TODO:Labelを取得する
fn eq() -> &'static str{
    "@SP\n\
    AM=M-1\n\
    D=A\n\
    @SP\n\
    AM=M-1\n\
    D=M-D\n\
    @labelA\n\
    D;JEQ\n\
    M=0\n\
    @labelB\n\
    (labelA)\n\
    M=-1\n\
    (labelB)\n\
    @SP\n\
    M=M+1\n"
}
// TODO:Labelを取得する
fn gt() -> &'static str{
    "@SP\n\
    AM=M-1\n\
    D=A\n\
    @SP\n\
    AM=M-1\n\
    D=M-D\n\
    @labelA\n\
    D;JGT\n\
    M=0\n\
    @labelB\n\
    (labelA)\n\
    M=-1\n\
    (labelB)\n\
    @SP\n\
    M=M+1\n"
}
// TODO:Labelを取得する
fn lt() -> &'static str{
    "@SP\n\
    AM=M-1\n\
    D=A\n\
    @SP\n\
    AM=M-1\n\
    D=M-D\n\
    @labelA\n\
    D;JLT\n\
    M=0\n\
    @labelB\n\
    (labelA)\n\
    M=-1\n\
    (labelB)\n\
    @SP\n\
    M=M+1\n"
}
