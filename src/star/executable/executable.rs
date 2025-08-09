use std::io;
use std::io::Write;

use std::mem::transmute;

use crate::debugger;
use crate::star::debuggable::*;
use crate::star::generateable::*;
use crate::star::math::*;
use crate::star::utils::*;

use crate::star::core::*;

use crate::interfaces;

pub trait Executable {
    fn execute(&mut self);

    fn increment_program_counter(&mut self);

    fn store_on_data_memory(&mut self, address: u16, value: u8) -> Result<(), String>;

    fn load_from_data_memory(&self, address: u16) -> Result<u8, String>;
}

/// Manipula operações de leitura de string da entrada padrão e armazena na memória de dados.
///
/// - Caso `self.registers.aux1 == 14`:
///   - Lê uma linha da entrada padrão (stdin), remove espaços em branco das extremidades e armazena os caracteres na memória de dados a partir do endereço especificado em `self.registers.aux2`.
///   - O número máximo de caracteres a serem armazenados é definido por `self.registers.aux3`.
///   - Não adiciona terminador nulo (`\0`) ao final da string.
///   - Se a string de entrada for maior que o limite, ela é truncada.
///   - Se o endereço de memória exceder os limites, ocorre um erro.
///   - Armazena o tamanho da string lida em `self.registers.aux2`
///
/// - Caso `self.registers.aux1 == 15`:
///   - Lê uma linha da entrada padrão (stdin), remove espaços em branco das extremidades e armazena os caracteres na memória de dados a partir do endereço especificado em `self.registers.aux2`.
///   - O número máximo de bytes a serem escritos é definido por `self.registers.aux3`.
///   - Sempre adiciona um terminador nulo (`\0`) ao final da string armazenada, desde que o tamanho máximo (`aux3`) seja maior que zero.
///   - Se a string de entrada for maior que o limite permitido (considerando o espaço para o terminador nulo), ela é truncada.
///   - Se o endereço de memória exceder os limites, ocorre um erro.
///
/// Exemplos de comportamento para o caso 15:
/// - Se `aux3 == 0`, nada é armazenado.
/// - Se `aux3 == 6` e a entrada for "Hello", armazena "Hello\0".
/// - Se `aux3 == 3` e a entrada for "Hello World", armazena "He\0".

impl Executable for Star {
    fn execute(&mut self) {
        let instruction_memory_len = match u16::try_from(self.instruction_memory.len()) {
            Ok(len) => len,
            Err(_) => {
                debugger::exit_with_error(
                    "Instruction memory length exceeds maximum size of 16 bits",
                );
                unreachable!();
            }
        };
        'execution_loop: while self.registers.program_counter < instruction_memory_len {
            //let (instruction_format, instruction_position) = match self.instruction_memory.get(self.registers.program_counter as usize) {
            //    Some(instr) => (instr.format, instr.position),
            //    None => {
            //        self.exit_with_error("Program counter out of bounds");
            //        break;
            //    }
            //};

            let instr_index: usize = (self.registers.program_counter as usize) * 2;
            let instruction_position_option = self
                .position_memory
                .get(self.registers.program_counter as usize)
                .cloned();
            let (instr_high, instr_low) = (
                match self.instruction_memory.get(instr_index) {
                    Some(byte) => *byte,
                    None => break 'execution_loop,
                },
                match self.instruction_memory.get(instr_index + 1) {
                    Some(byte) => *byte,
                    None => break 'execution_loop,
                },
            );

            let instruction_format: u16 =
                unsafe { transmute::<(u8, u8), u16>((instr_low, instr_high)) };

            // ==== PERFORMANCE DETECTOR ====
            /*
               This part of the code is used to detect non
               state alterable instructions, such as NOPs.
               If the instruction does not alter the state of
               the vm, it will not be decoded and executed.

               This is used to improve performance, as the
               instruction decoder is a costly operation.
            */
            if instruction_format == 0b_0000_0000_0000_0000 {
                // NOP instruction, just increment the program counter
                self.increment_program_counter();
                continue;
            }

            // ==== INSTRUCTION DECODER ====
            match Format::from_u16(instruction_format) {
                Format::Trinity => {
                    match defold_trinity(instruction_format) {
                        Some((instruction, reg1, reg2, reg3)) => {
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
                                    self.registers.carry = if is_carry { 0xFFFF } else { 0 };

                                    self.increment_program_counter();
                                }

                                Instruction::And | Instruction::Or | Instruction::Xor => {
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

                                Instruction::Shl | Instruction::Shr => {
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
                                | Instruction::Bltur => {
                                    let reg1_v = self.registers.get(reg1);
                                    let reg2_v = self.registers.get(reg2);
                                    let reg3_v = self.registers.get(reg3);

                                    let condition: bool = match instruction {
                                        Instruction::Beqr => reg1_v == reg2_v,
                                        Instruction::Bneqr => reg1_v != reg2_v,
                                        Instruction::Bgtr => 
                                            u16::cast_signed(reg1_v)
                                                > u16::cast_signed(reg2_v)
                                        ,
                                        Instruction::Bltr =>
                                            u16::cast_signed(reg1_v)
                                                < u16::cast_signed(reg2_v)
                                        ,
                                        Instruction::Bgtur => reg1_v > reg2_v,
                                        Instruction::Bltur => reg1_v < reg2_v,
                                        _ => unreachable!(),
                                    };

                                    if condition {
                                        match self.registers.program_counter.checked_add(1) {
                                            Some(ra) => self.registers.return_address = ra,
                                            None => self.exit_with_optional_positional_error(
                                                "Return address overflow",
                                                instruction_position_option,
                                            ),
                                        }

                                        self.registers.program_counter =
                                            self.registers.program_counter.wrapping_add(reg3_v);
                                    } else {
                                        self.increment_program_counter();
                                    }
                                }

                                _ => unimplemented!(),
                            }
                        }

                        None => self.exit_with_optional_positional_error(
                            "Invalid instruction format for Trinity",
                            instruction_position_option,
                        ),
                    }
                }

                Format::Hime => match defold_hime(instruction_format) {
                    Some((instruction, reg, imm)) => match instruction {
                        Instruction::Lai => {
                            let reg_v = self.registers.get(reg);
                            let regv_low = unsafe { transmute::<u16, (u8, u8)>(reg_v).0 };

                            let new_value = unsafe { transmute::<(u8, u8), u16>((regv_low, imm)) };

                            self.registers.set(reg, new_value);

                            self.increment_program_counter();
                        }
                        Instruction::Lli => {
                            let reg_v = self.registers.get(reg);
                            let regv_high = unsafe { transmute::<u16, (u8, u8)>(reg_v).1 };

                            let new_value = unsafe { transmute::<(u8, u8), u16>((imm, regv_high)) };

                            self.registers.set(reg, new_value);

                            self.increment_program_counter();
                        }
                        _ => unreachable!(),
                    },
                    None => self.exit_with_optional_positional_error(
                        "Invalid instruction format for Hime",
                        instruction_position_option,
                    ),
                },

                Format::Pair => match defold_pair(instruction_format) {
                    Some((instruction, reg1, reg2)) => match instruction {
                        Instruction::Mulhl => {
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
                            let reg1_v: i16 =
                                 u16::cast_signed(self.registers.get(reg1));
                            let reg2_v: i16 =
                                u16::cast_signed(self.registers.get(reg2));

                            if reg2_v == 0 {
                                self.registers.high = 0xFFFF;
                                self.registers.low = 0xFFFF;
                            } else {
                                let res = reg1_v.wrapping_div(reg2_v);
                                let rem = reg1_v.wrapping_rem(reg2_v);
                                self.registers.high = i16::cast_unsigned(rem);
                                self.registers.low = i16::cast_unsigned(res);
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
                            self.registers.set(reg1, !reg2_v);
                            self.increment_program_counter();
                        }

                        Instruction::Xlb => {
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

                        Instruction::Lab | Instruction::Llb => {
                            let reg1_v = self.registers.get(reg1);
                            let reg2_v = self.registers.get(reg2);

                            let (low, high) = unsafe { transmute::<u16, (u8, u8)>(reg1_v) };
                            match self.load_from_data_memory(reg2_v) {
                                Ok(value) => {
                                    let v = match instruction {
                                        Instruction::Lab => unsafe {
                                            transmute::<(u8, u8), u16>((low, value))
                                        },
                                        Instruction::Llb => unsafe {
                                            transmute::<(u8, u8), u16>((value, high))
                                        },
                                        _ => unreachable!(),
                                    };

                                    self.registers.set(reg1, v);
                                }
                                Err(e) => self.exit_with_optional_positional_error(
                                    e.as_str(),
                                    instruction_position_option,
                                ),
                            }
                            self.increment_program_counter();
                        }

                        Instruction::Sab | Instruction::Slb => {
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
                                Err(e) => self.exit_with_optional_positional_error(
                                    e.as_str(),
                                    instruction_position_option,
                                ),
                            }

                            self.increment_program_counter();
                        }

                        _ => unreachable!(),
                    },
                    None => self.exit_with_optional_positional_error(
                        "Invalid instruction format for Pair",
                        instruction_position_option,
                    ),
                },

                Format::Clover => match defold_clover(instruction_format) {
                    Some((instruction, reg)) => match instruction {
                        Instruction::J => {
                            let reg_v = self.registers.get(reg);
                            match self.registers.program_counter.checked_add(1) {
                                Some(ra) => self.registers.return_address = ra,
                                None => self.exit_with_optional_positional_error(
                                    "Return address overflow",
                                    instruction_position_option,
                                ),
                            }
                            self.registers.program_counter = reg_v;
                        }
                        _ => unreachable!(),
                    },
                    None => self.exit_with_optional_positional_error(
                        "Invalid instruction format for Clover",
                        instruction_position_option,
                    ),
                },

                Format::Ark => {
                    match defold_ark(instruction_format) {
                        Some(instruction) => {
                            match instruction {
                                Instruction::Mcall => {
                                    if interfaces::default(self) {
                                        break 'execution_loop;
                                    }

                                    self.increment_program_counter(); 
                                }
                                _ => unreachable!(),
                            }
                        }
                        None => self.exit_with_optional_positional_error(
                            "Invalid instruction format for Ark",
                            instruction_position_option,
                        ),
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
                debugger::exit_with_error("Program counter overflow");
            }
        }
    }

    fn store_on_data_memory(&mut self, address: u16, value: u8) -> Result<(), String> {
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
