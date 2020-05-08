use crate::lib::*;

pub fn dest(dest_type: DestType) -> [i32;3] {
    return match dest_type {
        DestType::Null => [0, 0, 0],
        DestType::M => [0, 0, 1],
        DestType::D => [0, 1, 0],
        DestType::MD => [0, 1, 1],
        DestType::A => [1, 0, 0],
        DestType::AM => [1, 0, 1],
        DestType::AD => [1, 1, 0],
        DestType::AMD => [1, 1, 1]
    }
}

pub fn jump(jump_type: JumpType) -> [i32;3]{
    return match jump_type {
        JumpType::Null => [0, 0, 0],
        JumpType::JGT => [0, 0, 1],
        JumpType::JEQ => [0, 1, 0],
        JumpType::JGE => [0, 1, 1],
        JumpType::JLT => [1, 0, 0],
        JumpType::JNE => [1, 0, 1],
        JumpType::JLE => [1, 1, 0],
        JumpType::JMP => [1, 1, 1],
    }
}

pub fn comp(comp_type: CompType) -> [i32;7]{
    return match comp_type {
        CompType::Zero => [0, 1, 0, 1, 0, 1, 0],
        CompType::One => [0, 1, 1, 1, 1, 1, 1],
        CompType::MinusOne => [0, 1, 1, 1, 0, 1, 0],
        CompType::D => [0, 0, 0, 1, 1, 0, 0],
        CompType::A => [0, 1, 1, 0, 0, 0, 0],
        CompType::NotD => [0, 0, 0, 1, 1, 0, 1],
        CompType::NotA => [0, 1, 1, 0, 0, 0, 1],
        CompType::MinusD => [0, 0, 0, 1, 1, 1, 1],
        CompType::MinusA => [0, 1, 1, 0, 0, 1, 1],
        CompType::DPlusOne=> [0, 0, 1, 1, 1, 1, 1] ,
        CompType::APlusOne=> [0, 1, 1, 0, 1, 1, 1]  ,
        CompType::DMinusOne => [0, 0, 0, 1, 1, 1, 0],
        CompType::AMinusOne => [0, 1, 1, 0, 0, 1, 0],
        CompType::DPlusA => [0, 0, 0, 0, 0, 1, 0],
        CompType::DMinusA => [0, 1, 0, 0, 0, 1, 1],
        CompType::AMinusD => [0, 0, 0, 0, 1, 1, 1],
        CompType::DAndA => [0, 0, 0, 0, 0, 0, 0],
        CompType::DOrA => [0, 0, 1, 0, 1, 0, 1],
        CompType::M => [1, 1, 1, 0, 0, 0, 0],
        CompType::NotM => [1, 1, 1, 0, 0, 0, 1],
        CompType::MinusM => [1, 1, 1, 0, 0, 1, 1],
        CompType::MPlusOne => [1, 1, 1, 0, 1, 1, 1],
        CompType::MMinusOne => [1, 1, 1, 0, 0, 1, 0],
        CompType::DPlusM => [1, 0, 0, 0, 0, 1, 0],
        CompType::DMinusM => [1, 1, 0, 0, 0, 1, 1],
        CompType::MMinusD => [1, 0, 0, 0, 1, 1, 1],
        CompType::DAndM => [1, 0, 0, 0, 0, 0, 0],
        CompType::DOrM => [1, 0, 1, 0, 1, 0, 1],
    }
}