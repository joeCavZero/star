use std::io;
use std::io::Write;
use std::mem::transmute;

use crate::debugger;
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
        let instruction_memory_len = match u16::try_from( self.instruction_memory.len() ) {
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

            let instr_index: usize = (self.registers.program_counter as usize) * 2 ;
            let instruction_position_option = self.position_memory.get(self.registers.program_counter as usize).cloned();
            let (instr_high, instr_low ) = (
                match self.instruction_memory.get( instr_index ) {
                    Some(byte) => *byte,
                    None => break 'execution_loop,
                },
                match self.instruction_memory.get( instr_index + 1 ) {
                    Some(byte) => *byte,
                    None => break 'execution_loop,
                },
            );
            
            let instruction_format = unsafe { transmute::<(u8, u8), u16>((instr_low, instr_high)) };

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
                            let reg3_v = self.registers.get(reg3);

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
                                match self.registers.program_counter.checked_add(1) {
                                    Some(ra) => {
                                        self.registers.return_address = ra
                                    }
                                    None => self.exit_with_optional_positional_error(
                                        "Return address overflow",
                                        instruction_position_option,
                                    ),
                                }

                                self.registers.program_counter = self.registers.program_counter.wrapping_add(reg3_v);
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
                                Err(e) => self.exit_with_optional_positional_error(e.as_str(), instruction_position_option),
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
                                Err(e) => self.exit_with_optional_positional_error(e.as_str(), instruction_position_option),
                            }
                            
                            self.increment_program_counter();
                        }

                        _ => unreachable!(),
                    }
                    
                }

                Format::Clover => {
                    let (instruction, reg) = defold_clover(instruction_format);
                    match instruction {
                        Instruction::J => {
                            let reg_v = self.registers.get(reg);
                            match self.registers.program_counter.checked_add(1) {
                                Some(ra) => {
                                    self.registers.return_address = ra
                                }
                                None => self.exit_with_optional_positional_error(
                                    "Return address overflow",
                                    instruction_position_option,
                                ),
                            }
                            self.registers.program_counter = reg_v;
                            
                        }
                        _ => unreachable!(),
                    }
                }

                Format::Ark => {
                    let instruction = defold_ark(instruction_format);
                    match instruction {
                        Instruction::Mcall => {
                            match self.registers.aux1 {
                                1 => { // print unsigned byte
                                    let low: u8 = unsafe { transmute::<u16, (u8, u8)>(self.registers.aux2).0 };
                                    print!("{}", low);
                                    io::stdout().flush().unwrap();

                                }
                                2 => { // print signed byte
                                    let low: u8 = unsafe { transmute::<u16, (u8, u8)>(self.registers.aux2).0 };
                                    let v: i8 = unsafe { transmute::<u8, i8>(low) };
                                    print!("{}", v);
                                    io::stdout().flush().unwrap();
                                }
                                3 => { // print unsigned word
                                    print!("{}", self.registers.aux2);
                                    io::stdout().flush().unwrap();
                                }
                                4 => { // print signed word
                                    let v: i16 = unsafe { transmute::<u16, i16>(self.registers.aux2) };
                                    print!("{}", v);
                                    io::stdout().flush().unwrap();
                                }
                                5 => { // print unsigned double
                                    let low: u16 = self.registers.aux2;
                                    let high: u16 = self.registers.aux3;
                                    let value: u32 = unsafe { transmute::<(u16, u16), u32>((low, high)) };
                                    print!("{}", value);
                                    io::stdout().flush().unwrap();
                                }
                                6 => { // print signed double
                                    let low: u16 = self.registers.aux2;
                                    let high: u16 = self.registers.aux3;
                                    let value: i32 = unsafe { transmute::<(u16, u16), i32>((low, high)) };
                                    print!("{}", value);
                                    io::stdout().flush().unwrap();
                                }
                                7 => { // print char
                                    let low: u8 = unsafe { transmute::<u16, (u8, u8)>(self.registers.aux2).0 };
                                    
                                    let c = low as char;
                                    print!("{}", c);
                                    io::stdout().flush().unwrap();
                                    
                                }
                                8 => { // print string with lenght (\0 not effects the string to print)
                                    let base_address = self.registers.aux2;
                                    let length = self.registers.aux3;

                                    let mut string: String = String::new();

                                    for i in 0..length {
                                        match base_address.checked_add(i) {
                                            Some(address) => {
                                                match self.load_from_data_memory(address) {
                                                    Ok(v) => {
                                                        string.push(v as char);
                                                    }
                                                    Err(_) => self.exit_with_optional_positional_error(
                                                        "String length exceeds memory bounds",
                                                        instruction_position_option,
                                                    ),
                                                }
                                            }
                                            None => self.exit_with_optional_positional_error(
                                                "String length exceeds memory bounds",
                                                instruction_position_option,
                                            ),
                                        }
                                    }

                                    print!("{}", string);
                                    io::stdout().flush().unwrap();
                                }
                                9 => { // print zero terminated string
                                    let base_address = self.registers.aux2;

                                    let mut string: String = String::new();
                                    let mut i = 0;
                                    loop {
                                        match base_address.checked_add(i) {
                                            Some(address) => {
                                                match self.load_from_data_memory(address) {
                                                    Ok(v) => {
                                                        if v == 0 {
                                                            break;
                                                        }
                                                        string.push(v as char);
                                                    }
                                                    Err(_) => self.exit_with_optional_positional_error(
                                                        "String length exceeds memory bounds",
                                                        instruction_position_option,
                                                    ),
                                                }
                                            }
                                            None => self.exit_with_optional_positional_error(
                                                "String length exceeds memory bounds",
                                                instruction_position_option,
                                            ),
                                        }
                                        i += 1;
                                    }

                                    print!("{}", string);
                                    io::stdout().flush().unwrap();
                                }
                                10 => { // read byte
                                    let mut input = String::new();
                                    io::stdin().read_line(&mut input).unwrap();
                                    match u8_from_string(input.trim().to_string()) {
                                        Ok(value) => {
                                            self.registers.aux2 = unsafe { transmute::<(u8, u8), u16>((value, 0)) };
                                        }
                                        Err(_) => {
                                            self.exit_with_optional_positional_error(
                                                "Invalid input for byte",
                                                instruction_position_option,
                                            );
                                        }
                                    }

                                }
                                11 => { // read word
                                    let mut input = String::new();
                                    io::stdin().read_line(&mut input).unwrap();
                                    match u16_from_string(input.trim().to_string()) {
                                        Ok(value) => {
                                            self.registers.aux2 = value;
                                        }
                                        Err(_) => {
                                            self.exit_with_optional_positional_error(
                                                "Invalid input for 16 bit word",
                                                instruction_position_option,
                                            );
                                        }
                                    }

                                }
                                12 => { // read double
                                    let mut input = String::new();
                                    io::stdin().read_line(&mut input).unwrap();
                                    match u32_from_string(input.trim().to_string()) {
                                        Ok(value) => {
                                            let (low, high) = unsafe { transmute::<u32, (u16, u16)>(value) };
                                            self.registers.aux2 = low;
                                            self.registers.aux3 = high;
                                        }
                                        Err(_) => {
                                            self.exit_with_optional_positional_error(
                                                "Invalid input for 32 bit double",
                                                instruction_position_option,
                                            );
                                        }
                                    }

                                }
                                13 => { // read character
                                    let mut input = String::new();
                                    io::stdin().read_line(&mut input).unwrap();
                                    let trimmed = input.trim();
                                    if trimmed.len() == 1 {
                                        let c = trimmed.chars().next().unwrap();
                                        self.registers.aux2 = c as u16;
                                    } else {
                                        self.exit_with_optional_positional_error(
                                            "Invalid input for character, it must be a single character",
                                            instruction_position_option,
                                        );
                                    }

                                }
                                14 => { // read string with a maximum length
                                    let mut input = String::new();
                                    io::stdin().read_line(&mut input).unwrap();
                                    let input = input.trim();
                                    let base_address_to_store = self.registers.aux2;
                                    let max_address_to_reach = match base_address_to_store.checked_add(self.registers.aux3) {
                                        Some(addr) => addr,
                                        None => {
                                            self.exit_with_optional_positional_error(
                                                "String length exceeds memory bounds",
                                                instruction_position_option,
                                            );
                                            unreachable!();
                                        }
                                    };

                                    let mut i = 0;
                                    for c in input.chars() {
                                        if i >= self.registers.aux3 as usize {
                                            break;
                                        }
                                        match base_address_to_store.checked_add(i as u16) {
                                            Some(address) => {
                                                if address >= max_address_to_reach {
                                                    break;
                                                }
                                                match self.store_on_data_memory(address, c as u8) {
                                                    Ok(_) => {}
                                                    Err(_) => self.exit_with_optional_positional_error(
                                                        "String exceeds memory bounds",
                                                        instruction_position_option,
                                                    ),
                                                }
                                            }
                                            None => self.exit_with_optional_positional_error(
                                                "String exceeds memory bounds",
                                                instruction_position_option,
                                            ),
                                        }
                                        i += 1;
                                    }

                                    // sets the aux2 register to the length of the string inserted
                                    self.registers.aux2 = match u16::try_from(input.len()) {
                                        Ok(len) => len,
                                        Err(_) => {
                                            self.exit_with_optional_positional_error(
                                                "String length exceeds maximum size of 16 bits",
                                                instruction_position_option,
                                            );
                                            unreachable!();
                                        }
                                    };
                                }
                                15 => { // read string zero with a maximum length, it always put a \0 at the end of the string inserted on memory
                                    /* Examples: 
                                        $aux3 = 0
                                        input: "Hello"
                                        memory: same as before, cause $aux3 = 0

                                        $aux3 = 6
                                        input: "Hello"
                                        memory: "Hello\0" (6 bytes, 5 characters + \0)

                                        $aux3 = 3
                                        input: "Hello World"
                                        memory: "He\0" (3 bytes, 2 characters + \0)
                                    */
                                    let mut input = String::new();
                                    io::stdin().read_line(&mut input).unwrap();
                                    let input = input.trim();
                                    let base_address_to_store = self.registers.aux2;
                                    let max_len = self.registers.aux3 as usize;

                                    if max_len == 0 {
                                        // Nothing LOL
                                    } else {
                                        let mut i = 0;
                                        for c in input.chars() {
                                            if i + 1 >= max_len {
                                                break;
                                            }
                                            match base_address_to_store.checked_add(i as u16) {
                                                Some(address) => {
                                                    match self.store_on_data_memory(address, c as u8) {
                                                        Ok(_) => {}
                                                        Err(_) => self.exit_with_optional_positional_error(
                                                            "String exceeds memory bounds",
                                                            instruction_position_option,
                                                        ),
                                                    }
                                                }
                                                None => self.exit_with_optional_positional_error(
                                                    "String exceeds memory bounds",
                                                    instruction_position_option,
                                                ),
                                            }
                                            i += 1;
                                        }
                                        // Always put \0 at the end
                                        match base_address_to_store.checked_add(i as u16) {
                                            Some(address) => {
                                                if i < max_len {
                                                    match self.store_on_data_memory(address, 0) {
                                                        Ok(_) => {}
                                                        Err(_) => self.exit_with_optional_positional_error(
                                                            "String exceeds memory bounds",
                                                            instruction_position_option,
                                                        ),
                                                    }
                                                }
                                            }
                                            None => self.exit_with_optional_positional_error(
                                                "String exceeds memory bounds",
                                                instruction_position_option,
                                            ),
                                        }
                                    }
                                }
                                16 => {
                                    break 'execution_loop;
                                }
                                17 => { // print instruction
                                    let target_instr_index: usize = (self.registers.aux2 as usize) * 2 ;
                                    let (target_instr_high, target_instr_low ) = (
                                        match self.instruction_memory.get( target_instr_index ) {
                                            Some(byte) => *byte,
                                            None => {
                                                self.exit_with_optional_positional_error(
                                                    "Target instruction out of bounds",
                                                    instruction_position_option,
                                                );
                                                unreachable!();
                                            }
                                        },
                                        match self.instruction_memory.get( target_instr_index + 1 ) {
                                            Some(byte) => *byte,
                                            None => {
                                                self.exit_with_optional_positional_error(
                                                    "Target instruction out of bounds",
                                                    instruction_position_option,
                                                );
                                                unreachable!();
                                            }
                                        },
                                    );
                                    
                                    let target_instruction_format = unsafe { transmute::<(u8, u8), u16>((target_instr_low, target_instr_high)) };

                                    print!("\n{:016b} ", target_instruction_format);
                                    io::stdout().flush().unwrap();

                                }
                                18 => { // sleep
                                    let millis: u64 = self.registers.aux2 as u64;
                                    std::thread::sleep(std::time::Duration::from_millis(millis));
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
                debugger::exit_with_error("Program counter overflow");
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


