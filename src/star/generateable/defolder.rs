use std::mem::transmute;

use crate::star::utils::*;

pub fn defold_trinity(fmt: u16) -> (Instruction, Register, Register, Register) {
    let instruction = Instruction::from_opcode(fmt & 0b_0000_0000_0000_1111);
    let reg1 = Register::from_code((fmt >> 4) & 0b_0000_0000_0000_1111);
    let reg2 = Register::from_code((fmt >> 8) & 0b_0000_0000_0000_1111);
    let reg3 = Register::from_code((fmt >> 12) & 0b_0000_0000_0000_1111);
    (instruction, reg1, reg2, reg3)
}

pub fn defold_hime(fmt: u16) -> (Instruction, Register, u8) {
    let instruction = Instruction::from_opcode(fmt & 0b_0000_0000_0000_1111);
    let reg = Register::from_code((fmt >> 4) & 0b_0000_0000_0000_1111);
    let raw_imm = (fmt >> 8) & 0b_0000_0000_1111_1111;
    let immediate = unsafe{transmute::<u16, (u8, u8)>(raw_imm).0};
    (instruction, reg, immediate)
}

pub fn defold_pair(fmt: u16) -> (Instruction, Register, Register) {
    let instruction = Instruction::from_opcode(fmt & 0b_0000_0000_1111_1111);
    let reg1 = Register::from_code((fmt >> 8) & 0b_0000_0000_0000_1111);
    let reg2 = Register::from_code((fmt >> 12) & 0b_0000_0000_0000_1111);
    (instruction, reg1, reg2)
}

pub fn defold_clover(fmt: u16) -> (Instruction, Register) {
    let instruction = Instruction::from_opcode(fmt & 0b_0000_1111_1111_1111);
    let reg = Register::from_code((fmt >> 12) & 0b_0000_0000_0000_1111);
    (instruction, reg)
}

pub fn defold_ark(fmt: u16) -> Instruction {
    Instruction::from_opcode(fmt & 0b_1111_1111_1111_1111)
}