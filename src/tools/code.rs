use crate::enums::{CompType, DestType, JumpType};

pub fn dest(dest_type: DestType) -> &'static str {
    return match dest_type {
        DestType::Null => "000",
        DestType::M => "001",
        DestType::D => "010",
        DestType::MD => "011",
        DestType::A => "100",
        DestType::AM => "101",
        DestType::AD => "110",
        DestType::AMD => "111",
    };
}

pub fn jump(jump_type: JumpType) -> &'static str {
    return match jump_type {
        JumpType::Null => "000",
        JumpType::JGT => "001",
        JumpType::JEQ => "010",
        JumpType::JGE => "011",
        JumpType::JLT => "100",
        JumpType::JNE => "101",
        JumpType::JLE => "110",
        JumpType::JMP => "111",
    };
}

pub fn comp(comp_type: CompType) -> &'static str {
    return match comp_type {
        CompType::Zero => "0101010",
        CompType::One => "0111111",
        CompType::MinusOne => "0111010",
        CompType::D => "0001100",
        CompType::A => "0110000",
        CompType::NotD => "0001101",
        CompType::NotA => "0110001",
        CompType::MinusD => "0001111",
        CompType::MinusA => "0110011",
        CompType::DPlusOne => "0011111",
        CompType::APlusOne => "0110111",
        CompType::DMinusOne => "0001110",
        CompType::AMinusOne => "0110010",
        CompType::DPlusA => "0000010",
        CompType::DMinusA => "0100011",
        CompType::AMinusD => "0000111",
        CompType::DAndA => "0000000",
        CompType::DOrA => "0010101",
        CompType::M => "1110000",
        CompType::NotM => "1110001",
        CompType::MinusM => "1110011",
        CompType::MPlusOne => "1110111",
        CompType::MMinusOne => "1110010",
        CompType::DPlusM => "1000010",
        CompType::DMinusM => "1010011",
        CompType::MMinusD => "1000111",
        CompType::DAndM => "1000000",
        CompType::DOrM => "1010101",
    };
}
