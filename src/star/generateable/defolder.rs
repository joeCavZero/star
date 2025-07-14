use std::mem::transmute;

use crate::star::utils::*;

pub fn defold_trinity(fmt: u16) -> Option<(Instruction, GeneralRegister, GeneralRegister, GeneralRegister)> {
    match Instruction::from_opcode(fmt & 0b_0000_0000_0000_1111) {
        Some(instruction) => {
            let reg1 = GeneralRegister::from_code((fmt >> 4) & 0b_0000_0000_0000_1111);
            let reg2 = GeneralRegister::from_code((fmt >> 8) & 0b_0000_0000_0000_1111);
            let reg3 = GeneralRegister::from_code((fmt >> 12) & 0b_0000_0000_0000_1111);
            Some((instruction, reg1, reg2, reg3))
        }
        None => None,
    }
}

pub fn defold_hime(fmt: u16) -> Option<(Instruction, GeneralRegister, u8)> {
    match Instruction::from_opcode(fmt & 0b_0000_0000_0000_1111) {
        Some(instruction) => {
            let reg = GeneralRegister::from_code((fmt >> 4) & 0b_0000_0000_0000_1111);
            let raw_imm = (fmt >> 8) & 0b_0000_0000_1111_1111;
            let immediate = unsafe { transmute::<u16, (u8, u8)>(raw_imm).0 };
            Some((instruction, reg, immediate))
        }
        None => None,
    }
}

pub fn defold_pair(fmt: u16) -> Option<(Instruction, GeneralRegister, GeneralRegister)> {
    match Instruction::from_opcode(fmt & 0b_0000_0000_1111_1111) {
        Some(instruction) => {
            let reg1 = GeneralRegister::from_code((fmt >> 8) & 0b_0000_0000_0000_1111);
            let reg2 = GeneralRegister::from_code((fmt >> 12) & 0b_0000_0000_0000_1111);
            Some((instruction, reg1, reg2))
        }
        None => None,
    }
}

pub fn defold_clover(fmt: u16) -> Option<(Instruction, GeneralRegister)> {
    match Instruction::from_opcode(fmt & 0b_0000_1111_1111_1111) {
        Some(instruction) => {
            let reg = GeneralRegister::from_code((fmt >> 12) & 0b_0000_0000_0000_1111);
            Some((instruction, reg))
        }
        None => None,
    }
}

pub fn defold_ark(fmt: u16) -> Option<Instruction> {
    Instruction::from_opcode(fmt & 0b_1111_1111_1111_1111)
}