
use std::mem::transmute;

use crate::star::core::*;
use crate::star::debuggable::Debugable;
use crate::star::generateable::*;
use crate::star::math::split_u16_to_strings;
use crate::star::math::u16_from_string;
use crate::star::math::u8_from_string;
use crate::star::parseable::*;
use crate::star::utils::*;

pub trait Generateable {
    fn generate(&mut self, ast: &Ast) -> usize;
    fn generate_data_memory(&mut self, ast: &Ast) -> usize;
    fn generate_instruction_memory(&mut self, ast: &Ast);
}

impl Generateable for Star {
    fn generate(&mut self, ast: &Ast) -> usize {
        let data_section_size = self.generate_data_memory(ast);
        self.generate_instruction_memory(ast);
        return data_section_size;
    }

    fn generate_data_memory(&mut self, ast: &Ast) -> usize {
        let mut is_data_section_empty = true;
        let mut data_memory_pointer: usize = 0;
        for data_camp in ast.data_field.iter() {
            is_data_section_empty = false;
            match data_camp.directive.token {
                Token::Directive(Directive::Byte)
                | Token::Directive(Directive::Word)
                => {

                    if let DataCampArg::Multiple(values) = &data_camp.arg {
                        for val_ptk in values.iter() {
                            if let Token::NumberLiteral(num_string) = &val_ptk.token {
                                match data_camp.directive.token {
                                    Token::Directive(Directive::Byte) => {
                                        match u8_from_string((*num_string).to_string()) {
                                            Ok(num) => {
                                                match self.data_memory.get_mut(data_memory_pointer) {
                                                    Some(byte) => {
                                                        *byte = num;
                                                        data_memory_pointer += 1;
                                                    }
                                                    None => {
                                                        self.exit_with_positional_error(
                                                            "Data memory overflow",
                                                            val_ptk.position,
                                                        );
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                self.exit_with_positional_error(
                                                    e.as_str(),
                                                    val_ptk.position,
                                                );
                                            }
                                        }
                                    }
                                    Token::Directive(Directive::Word) => {
                                        match u16_from_string((*num_string).to_string()) {
                                            Ok(num) => {
                                                let (low, high) = split_u16_to_strings(num);

                                                if let Some(byte1) = self.data_memory.get_mut(data_memory_pointer) {
                                                    *byte1 = u8_from_string(high).unwrap_or(0);
                                                } else {
                                                    self.exit_with_positional_error(
                                                        "Data memory overflow",
                                                        val_ptk.position,
                                                    );
                                                }

                                                data_memory_pointer += 1;

                                                if let Some(byte2) = self.data_memory.get_mut(data_memory_pointer) {
                                                    *byte2 = u8_from_string(low).unwrap_or(0);
                                                } else {
                                                    self.exit_with_positional_error(
                                                        "Data memory overflow",
                                                        val_ptk.position,
                                                    );
                                                }

                                                data_memory_pointer += 1;
                                            }
                                            Err(e) => {
                                                self.exit_with_positional_error(
                                                    e.as_str(),
                                                    val_ptk.position,
                                                );
                                            }
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            } 
                        }
                    } else {
                        unreachable!();
                    }
                }
                Token::Directive(Directive::Space) => {
                    if let DataCampArg::Unique(value) = data_camp.arg.clone() {
                        if let Token::NumberLiteral(num_string) = value.token {
                            match u16_from_string((*num_string).to_string()) {
                                Ok(num) => {
                                    data_memory_pointer += num as usize;
                                }
                                Err(e) => {
                                    self.exit_with_positional_error(
                                        e.as_str(),
                                        value.position,
                                    );
                                }
                            }
                        } else {
                            unreachable!();
                        }
                    } else {
                        unreachable!();
                    }
                }
                Token::Directive(Directive::String)
                | Token::Directive(Directive::Stringz) => {
                    if let DataCampArg::Unique(value) = data_camp.arg.clone() {
                        if let Token::StringLiteral(string) = value.token {
                            let string_bytes = string.as_bytes();
                            for &byte in string_bytes.iter() {
                                if let Some(data_byte) = self.data_memory.get_mut(data_memory_pointer) {
                                    *data_byte = byte;
                                    data_memory_pointer += 1;
                                } else {
                                    self.exit_with_positional_error(
                                        "Data memory overflow",
                                        value.position,
                                    );
                                }
                            }

                            // If it's a null-terminated string, add a null byte
                            if data_camp.directive.token == Token::Directive(Directive::Stringz) {
                                if let Some(data_byte) = self.data_memory.get_mut(data_memory_pointer) {
                                    *data_byte = 0; // Null terminator
                                    data_memory_pointer += 1;
                                } else {
                                    self.exit_with_positional_error(
                                        "Data memory overflow",
                                        value.position,
                                    );
                                }
                            }
                        } else {
                            unreachable!();
                        }
                    } else {
                        unreachable!();
                    }
                }
                Token::Directive(Directive::Checkpoint) => {
                    if let DataCampArg::Empty = data_camp.arg {
                        // Nothing to do here, just a checkpoint
                    } else {
                        self.exit_with_positional_error(
                            "Checkpoint directive does not accept arguments",
                            data_camp.directive.position,
                        );
                    }
                }
                _ => unreachable!(),
            }
        }
    
        if is_data_section_empty {
            return 0;
        } else {
            return data_memory_pointer;
        }
    }

    fn generate_instruction_memory(&mut self, ast: &Ast) {
        for instr_camp in ast.instr_field.iter() {
            if let Token::Instruction(instruction) = instr_camp.instruction.token.clone() {
                match instruction.format() {
                    Format::Trinity => {
                        if let Sequence::Three(reg_ptk_1, reg_ptk_2, reg_ptk_3) = instr_camp.sequence.clone() {
                            if let (Token::GeneralRegister(reg1), Token::GeneralRegister(reg2), Token::GeneralRegister(reg3)) = (reg_ptk_1.token.clone(), reg_ptk_2.token.clone(), reg_ptk_3.token.clone()) {
                                let format: u16 = fold_trinity(
                                    instruction,
                                    reg1,
                                    reg2,
                                    reg3,
                                );

                                self.position_memory.push(instr_camp.instruction.position);

                                let (instr_low, instr_high) = unsafe { transmute::<u16, (u8, u8)>(format) };
                                self.instruction_memory.push(instr_high);
                                self.instruction_memory.push(instr_low);
                            } else {
                                unreachable!();
                            }
                        } else {
                            unreachable!();
                        }
                    }
                    Format::Hime => {
                        if let Sequence::Two(reg_ptk, imm_ptk) = instr_camp.sequence.clone() {
                            if let (Token::GeneralRegister(reg), Token::NumberLiteral(imm_string)) = (reg_ptk.token.clone(), imm_ptk.token.clone()) {
                                match u8_from_string(imm_string) {
                                    Ok(imm) => {
                                        let format: u16 = fold_hime(
                                            instruction,
                                            reg,
                                            imm,
                                        );
                                        
                                        self.position_memory.push(instr_camp.instruction.position);

                                        let (instr_low, instr_high) = unsafe { transmute::<u16, (u8, u8)>(format) };
                                        self.instruction_memory.push(instr_high);
                                        self.instruction_memory.push(instr_low);
                                    }
                                    Err(e) => {
                                        self.exit_with_positional_error(
                                            e.as_str(),
                                            imm_ptk.position,
                                        );
                                    }
                                }
                            } else {
                                unreachable!();
                            }
                        } else {
                            unreachable!();
                        }
                    }
                    Format::Pair => {
                        if let Sequence::Two(reg_ptk_1, reg_ptk_2) = instr_camp.sequence.clone() {
                            if let (Token::GeneralRegister(reg1), Token::GeneralRegister(reg2)) = (reg_ptk_1.token.clone(), reg_ptk_2.token.clone()) {
                                let format: u16 = fold_pair(
                                    instruction,
                                    reg1,
                                    reg2,
                                );
                                
                                self.position_memory.push(instr_camp.instruction.position);

                                let (instr_low, instr_high) = unsafe { transmute::<u16, (u8, u8)>(format) };
                                self.instruction_memory.push(instr_high);
                                self.instruction_memory.push(instr_low);
                            } else {
                                unreachable!();
                            }
                        } else {
                            unreachable!();
                        }
                    }
                    Format::Clover => {
                        if let Sequence::One(reg_ptk) = instr_camp.sequence.clone() {
                            if let Token::GeneralRegister(reg) = reg_ptk.token.clone() {
                                let format: u16 = fold_clover(
                                    instruction,
                                    reg,
                                );
                                
                                self.position_memory.push(instr_camp.instruction.position);

                                let (instr_low, instr_high) = unsafe { transmute::<u16, (u8, u8)>(format) };
                                self.instruction_memory.push(instr_high);
                                self.instruction_memory.push(instr_low);
                            } else {
                                unreachable!();
                            }
                        } else {
                            unreachable!();
                        }
                    }
                    Format::Ark => {
                        if let Sequence::Zero = instr_camp.sequence.clone() {
                            let format: u16 = fold_ark(instruction);
                            
                            self.position_memory.push(instr_camp.instruction.position);

                            let (instr_low, instr_high) = unsafe { transmute::<u16, (u8, u8)>(format) };
                            self.instruction_memory.push(instr_high);
                            self.instruction_memory.push(instr_low);
                        } else {
                            unreachable!();
                        }
                    }
                }
            }
        }
    }
}