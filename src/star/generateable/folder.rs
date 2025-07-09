use crate::star::utils::*;

pub fn fold_trinity(instruction: Instruction, reg1: GeneralRegister, reg2: GeneralRegister, reg3: GeneralRegister) -> u16 {
    let mut format = 0b0000_0000_0000_0000;
    format |= instruction.opcode();
    format |= reg1.code() << 4;
    format |= reg2.code() << 8;
    format |= reg3.code() << 12;
    format
}

pub fn fold_hime(instruction: Instruction, reg1: GeneralRegister, immediate: u8) -> u16 {
    let mut format = 0b0000_0000_0000_0000;
    format |= instruction.opcode();
    format |= reg1.code() << 4;
    format |= (immediate as u16) << 8;
    format
}

pub fn fold_pair(instruction: Instruction, reg1: GeneralRegister, reg2: GeneralRegister) -> u16 {
    let mut format = 0b0000_0000_0000_0000;
    format |= instruction.opcode();
    format |= reg1.code() << 8;
    format |= reg2.code() << 12;
    format
}

pub fn fold_clover(instruction: Instruction, reg: GeneralRegister) -> u16 {
    let mut format = 0b0000_0000_0000_0000;
    format |= instruction.opcode();
    format |= reg.code() << 12;
    format
}

pub fn fold_ark(instruction: Instruction) -> u16 {
    let mut format = 0b0000_0000_0000_0000;
    format |= instruction.opcode();
    format
}