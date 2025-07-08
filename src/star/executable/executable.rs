use std::io;
use std::io::Write;
use std::mem::transmute;

use crate::star::debuggable::*;
use crate::star::generateable::*;
use crate::star::math::*;
use crate::star::utils::*;

use crate::star::core::*;



pub trait Executable {
    fn execute(&mut self);

    fn increment_program_counter(&mut self);

    fn store_on_data_memory(&mut self, address: u16, value: u8) -> Result<(), String>;

    fn load_from_data_memory(&self, address: u16) -> Result<u8, String>;
}

impl Executable for Star {
    fn execute(&mut self) {
        let instruction_memory_len = match u16::try_from( self.instruction_memory.len() ) {
            Ok(len) => len,
            Err(_) => {
                self.exit_with_error(
                    "Instruction memory length exceeds maximum size of 16 bits",
                );
                unreachable!();
            }
        };
        'execution_loop: while self.registers.program_counter < instruction_memory_len {
            let (instruction_format, instruction_position) = match self.instruction_memory.get(self.registers.program_counter as usize) {
                Some(instr) => (instr.format, instr.position),
                None => {
                    self.exit_with_error("Program counter out of bounds");
                    break;
                }
            };

            match Format::from_u16(instruction_format) {
                Format::Trinity => {
                    let (instruction, reg1, reg2, reg3) = defold_trinity(instruction_format);
                
                    match instruction {
                        Instruction::Add => {
                            let reg2_v = self.registers.get(reg2);
                            let reg3_v = self.registers.get(reg3);
                            let (res, is_carry) = reg2_v.overflowing_add(reg3_v);
                            self.registers.set(reg1, res);
                            self.registers.carry = if is_carry { 1 } else { 0 };
                            
                            self.increment_program_counter();
                        }
                        Instruction::Sub => {
                            let reg2_v = self.registers.get(reg2);
                            let reg3_v = self.registers.get(reg3);
                            let (res, is_carry) = reg2_v.overflowing_sub(reg3_v);
                            self.registers.set(reg1, res);
                            self.registers.carry = if is_carry { 0xFFFF } else {0};

                            self.increment_program_counter();
                        }

                        Instruction::And 
                        | Instruction::Or
                        | Instruction::Xor
                        => {
                            let reg2_v = self.registers.get(reg2);
                            let reg3_v = self.registers.get(reg3);
                            let res = match instruction {
                                Instruction::And => reg2_v & reg3_v,
                                Instruction::Or => reg2_v | reg3_v,
                                Instruction::Xor => reg2_v ^ reg3_v,
                                _ => unreachable!(),
                            };
                            self.registers.set(reg1, res);

                            self.increment_program_counter();
                        }

                        Instruction::Shl 
                        | Instruction::Shr
                        => {
                            let reg2_v: u16 = self.registers.get(reg2);
                            let reg3_v: u16 = self.registers.get(reg3);
                            
                            let (res, carry) = match instruction {
                                Instruction::Shl => shift_left_with_carry(reg2_v, reg3_v),
                                Instruction::Shr => shift_right_with_carry(reg2_v, reg3_v),
                                _ => unreachable!(),
                            };
                            self.registers.set(reg1, res);
                            self.registers.carry = carry;

                            self.increment_program_counter();
                        }

                        // ==== Branches ====
                        Instruction::Beqr 
                        | Instruction::Bneqr
                        | Instruction::Bgtr
                        | Instruction::Bltr
                        | Instruction::Bgtur
                        | Instruction::Bltur
                        => {
                            let reg1_v = self.registers.get(reg1);
                            let reg2_v = self.registers.get(reg2);
                            let reg3_v = unsafe { transmute::<u16, i16>(self.registers.get(reg3)) };

                            let condition: bool = match instruction {
                                Instruction::Beqr => reg1_v == reg2_v,
                                Instruction::Bneqr => reg1_v != reg2_v,
                                Instruction::Bgtr => unsafe{ transmute::<u16, i16>(reg1_v) > transmute::<u16, i16>(reg2_v) },
                                Instruction::Bltr => unsafe{ transmute::<u16, i16>(reg1_v) < transmute::<u16, i16>(reg2_v) },
                                Instruction::Bgtur => reg1_v > reg2_v,
                                Instruction::Bltur => reg1_v < reg2_v,
                                _ => unreachable!(),
                            };

                            if condition {
                                if reg3_v >= 0 {
                                    match self.registers.program_counter.checked_add(reg3_v as u16) {
                                        Some(new_pc) => {
                                            self.registers.program_counter = new_pc;
                                        }
                                        None => {
                                            self.exit_with_error("Program counter overflow");
                                        }
                                    }
                                } else {
                                    match self.registers.program_counter.checked_sub((-reg3_v) as u16) {
                                        Some(new_pc) => {
                                            self.registers.program_counter = new_pc;
                                        }
                                        None => {
                                            self.exit_with_error("Program counter underflow");
                                        }
                                    }
                                }
                            } else {
                                self.increment_program_counter();
                            }
                        }

                        _ => unimplemented!(),
                    }
                }

                Format::Hime => {
                    let (instruction, reg, imm) = defold_hime(instruction_format);

                    match instruction {
                        Instruction::Lai => {
                            let reg_v = self.registers.get(reg);
                            let regv_low = unsafe { transmute::<u16, (u8, u8)>(reg_v).0 };
                            
                            let new_value = unsafe { transmute::<(u8, u8), u16>((regv_low, imm )) };

                            self.registers.set(reg, new_value);

                            self.increment_program_counter();
                        }
                        Instruction::Lli => {
                            let reg_v = self.registers.get(reg);
                            let regv_high = unsafe { transmute::<u16, (u8, u8)>(reg_v).1 };
                            
                            let new_value = unsafe { transmute::<(u8, u8), u16>((imm, regv_high )) };

                            self.registers.set(reg, new_value);

                            self.increment_program_counter();
                        }
                        _ => unreachable!(),
                    }
                }
                
                Format::Pair => {
                    let (instruction, reg1, reg2) = defold_pair(instruction_format);
                    match instruction {
                        Instruction::Mulhl 
                        => {
                            let reg1_v: u32 = extend_sign_from_u16_to_u32(self.registers.get(reg1));
                            let reg2_v: u32 = extend_sign_from_u16_to_u32(self.registers.get(reg2));
                            
                            let res = reg1_v.wrapping_mul(reg2_v);
                            let (low, high) = unsafe { transmute::<u32, (u16, u16)>(res) };
                            self.registers.high = high;
                            self.registers.low = low;

                            self.increment_program_counter();
                        }
                        
                        Instruction::Muluhl => {
                            let reg1_v: u32 = extend_zero_from_u16_to_u32(self.registers.get(reg1));
                            let reg2_v: u32 = extend_zero_from_u16_to_u32(self.registers.get(reg2));
                            
                            let res = reg1_v.wrapping_mul(reg2_v);
                            let (low, high) = unsafe { transmute::<u32, (u16, u16)>(res) };
                            self.registers.high = high;
                            self.registers.low = low;

                            self.increment_program_counter();
                        }

                        Instruction::Divhl => {
                            let reg1_v: i16 = unsafe{ transmute::<u16, i16>(self.registers.get(reg1)) };
                            let reg2_v: i16 = unsafe{ transmute::<u16, i16>(self.registers.get(reg2)) };

                            if reg2_v == 0 {
                                self.registers.high = 0xFFFF;
                                self.registers.low = 0xFFFF;
                            } else {
                                let res = reg1_v.wrapping_div(reg2_v);
                                let rem = reg1_v.wrapping_rem(reg2_v);
                                self.registers.high = unsafe { transmute::<i16, u16>(rem) };
                                self.registers.low = unsafe { transmute::<i16, u16>(res) };                                
                            }
                            self.increment_program_counter();
                        }

                        Instruction::Divuhl => {
                            let reg1_v: u16 = self.registers.get(reg1);
                            let reg2_v: u16 = self.registers.get(reg2);

                            if reg2_v == 0 {
                                self.registers.high = 0xFFFF;
                                self.registers.low = 0xFFFF;
                            } else {
                                let res = reg1_v.wrapping_div(reg2_v);
                                let rem = reg1_v.wrapping_rem(reg2_v);
                                self.registers.high = rem;
                                self.registers.low = res;                                
                            }
                            self.increment_program_counter();
                        }

                        Instruction::Not => {
                            let reg2_v = self.registers.get(reg2);
                            self.registers.set(reg1, !reg2_v );
                            self.increment_program_counter();
                        }

                        Instruction::Xb => {
                            let reg2_v = self.registers.get(reg2);
                            
                            let (low, _) = unsafe { transmute::<u16, (u8, u8)>(reg2_v) };
                            let mut high: u8 = 0b_0000_0000;
                            if reg2_v & 0b_0000_0000_1000_0000 != 0 {
                                high = 0b_1111_1111;
                            }
                            let res = unsafe { transmute::<(u8, u8), u16>((low, high)) };
                            self.registers.set(reg1, res);
                            self.increment_program_counter();
                        }

                        Instruction::Lab 
                        | Instruction::Llb
                        => {
                            let reg1_v = self.registers.get(reg1);
                            let reg2_v = self.registers.get(reg2);

                            let (low, high) = unsafe { transmute::<u16, (u8, u8)>(reg1_v) };
                            match self.load_from_data_memory(reg2_v) {
                                Ok(value) => {
                                    let v = match instruction {
                                        Instruction::Lab => unsafe { transmute::<(u8, u8), u16>((low, value)) },
                                        Instruction::Llb => unsafe { transmute::<(u8, u8), u16>((value, high)) },
                                        _ => unreachable!(),
                                    };
                                    
                                    self.registers.set(reg1, v);
                                }
                                Err(e) => self.exit_with_positional_error(e.as_str(), instruction_position),
                            }
                            self.increment_program_counter();
                        }

                        Instruction::Sab
                        | Instruction::Slb
                        => {
                            let reg1_v = self.registers.get(reg1);
                            let reg2_v = self.registers.get(reg2);

                            let (low, high) = unsafe { transmute::<u16, (u8, u8)>(reg1_v) };
                            let value = match instruction {
                                Instruction::Sab => high,
                                Instruction::Slb => low,
                                _ => unreachable!(),
                            };

                            match self.store_on_data_memory(reg2_v, value) {
                                Ok(_) => {}
                                Err(e) => self.exit_with_positional_error(e.as_str(), instruction_position),
                            }
                            
                            self.increment_program_counter();
                        }

                        Instruction::Jar => { // jump absolute relative
                            let reg1_v = self.registers.get(reg1);
                            let reg2_v = self.registers.get(reg2);

                            let address = reg1_v.wrapping_add(reg2_v);
                            
                            self.registers.program_counter = address;
                        }
                        
                        _ => unreachable!(),
                    }
                    
                }

                Format::Clover => {
                    let (_instruction, _reg) = defold_clover(instruction_format);
                }

                Format::Ark => {
                    let instruction = defold_ark(instruction_format);
                    match instruction {
                        Instruction::Mcall => {
                            match self.registers.aux1 {
                                0 => { // print register as u16
                                    print!("{}", self.registers.aux2);
                                    io::stdout().flush().unwrap();
                                }
                                1 => { // print register as i16
                                    let value: i16 = unsafe { transmute::<u16, i16>(self.registers.aux2) };
                                    print!("{}", value);
                                    io::stdout().flush().unwrap();
                                }
                                2 => { // read 16 bit integer
                                    let mut input = String::new();
                                    io::stdin().read_line(&mut input).unwrap();
                                    match u16_from_string(input.trim().to_string()) {
                                        Ok(value) => {
                                            self.registers.aux2 = value;
                                        }
                                        Err(_) => {
                                            self.exit_with_positional_error(
                                                "Invalid input for 16 bit integer",
                                                instruction_position,
                                            );
                                        }
                                    }

                                }
                                3 => { // print register as char
                                    let value: u16 = self.registers.aux2;
                                    if value <= 255 {
                                        let c = value as u8 as char;
                                        print!("{}", c);
                                        io::stdout().flush().unwrap();
                                    } else {
                                        self.exit_with_positional_error(
                                            "Invalid character value",
                                            instruction_position,
                                        );
                                    }
                                }
                                4 => { // read character
                                    let mut input = String::new();
                                    io::stdin().read_line(&mut input).unwrap();
                                    let trimmed = input.trim();
                                    if trimmed.len() == 1 {
                                        let c = trimmed.chars().next().unwrap();
                                        self.registers.aux2 = c as u16;
                                    } else {
                                        self.exit_with_positional_error(
                                            "Invalid input for character",
                                            instruction_position,
                                        );
                                    }

                                }
                                10 => {
                                    break 'execution_loop;
                                }
                                _ => {}
                            }
                            self.increment_program_counter();
                        }
                        _ => unreachable!(),
                    }
                    
                    
                }
            }
            
        }
        io::stdout().flush().unwrap();
    }

    fn increment_program_counter(&mut self) {
        match self.registers.program_counter.checked_add(1) {
            Some(new_pc) => {
                self.registers.program_counter = new_pc;
            }
            None => {
                self.exit_with_error("Program counter overflow");
            }
        }
    }

    fn store_on_data_memory(&mut self, address: u16, value: u8) -> Result<(), String>{
        match self.data_memory.get_mut(address as usize) {
            Some(cell) => {
                *cell = value;
                Ok(())
            }
            None => Err(format!("Data memory address {} out of bounds", address)),
        }
    }

    fn load_from_data_memory(&self, address: u16) -> Result<u8, String> {
        match self.data_memory.get(address as usize) {
            Some(value) => Ok(*value),
            None => Err(format!("Data memory address {} out of bounds", address)),
        }
        
    }
}


