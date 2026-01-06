
use star::prelude::*;

use std::io;
use std::io::Write;
use std::mem::transmute;

pub struct Interface;

impl Interface {
    pub fn new() -> Self {
        Interface{}
    }
}

impl StarInterface for Interface {
    fn mcall(&mut self, s: &mut dyn StarMcallContext) -> bool {
        let registers = s.get_registers_mut().clone();

        match registers.aux1 {
            1 => {
                // print unsigned byte
                let low: u8 = unsafe { transmute::<u16, (u8, u8)>(registers.aux2).0 };
                print!("{low}");
                io::stdout().flush().unwrap();
            }
            2 => {
                // print signed byte
                let low: u8 = unsafe { transmute::<u16, (u8, u8)>(registers.aux2).0 };
                let v: i8 = u8::cast_signed(low);
                print!("{v}");
                io::stdout().flush().unwrap();
            }
            3 => {
                // print unsigned word
                print!("{}", registers.aux2);
                io::stdout().flush().unwrap();
            }
            4 => {
                // print signed word
                let v: i16 = u16::cast_signed(registers.aux2);
                print!("{v}");
                io::stdout().flush().unwrap();
            }
            5 => {
                // print unsigned double
                let value: u32 =
                    unsafe { transmute::<(u16, u16), u32>((registers.aux2, registers.aux3)) };
                print!("{value}");
                io::stdout().flush().unwrap();
            }
            6 => {
                // print signed double
                let value: i32 =
                    unsafe { transmute::<(u16, u16), i32>((registers.aux2, registers.aux3)) };
                print!("{value}");
                io::stdout().flush().unwrap();
            }
            7 => {
                // print char
                let low: u8 = unsafe { transmute::<u16, (u8, u8)>(registers.aux2).0 };
                print!("{}", low as char);
                io::stdout().flush().unwrap();
            }
            8 => {
                // print string with length
                let mut string = String::new();
                for i in 0..registers.aux3 {
                    let address = match registers.aux2.checked_add(i) {
                        Some(a) => a,
                        None => return true,
                    };
                    match s.get_data_memory_mut().load(address) {
                        Ok(v) => string.push(v as char),
                        Err(_) => return true,
                    }
                }
                print!("{string}");
                io::stdout().flush().unwrap();
            }
            9 => {
                // print zero terminated string
                let mut string = String::new();
                let mut i: u16 = 0;
                loop {
                    let address = match registers.aux2.checked_add(i) {
                        Some(a) => a,
                        None => return true,
                    };
                    let v = match s.get_data_memory_mut().load(address) {
                        Ok(v) => v,
                        Err(_) => return true,
                    };
                    if v == 0 {
                        break;
                    }
                    string.push(v as char);
                    i += 1;
                }
                print!("{string}");
                io::stdout().flush().unwrap();
            }
            10 => {
                // read byte
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match star::math::u8_from_string(input.trim().to_string()) {
                    Ok(value) => {
                        let v = unsafe { transmute::<(u8, u8), u16>((value, 0)) };
                        s.set_general_register_value(StarGeneralRegister::Aux2, v);
                    }
                    Err(_) => return true,
                }
            }
            11 => {
                // read word
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match star::math::u16_from_string(input.trim().to_string()) {
                    Ok(value) => {
                        s.set_general_register_value(StarGeneralRegister::Aux2, value);
                    }
                    Err(_) => return true,
                }
            }
            12 => {
                // read double
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match star::math::u32_from_string(input.trim().to_string()) {
                    Ok(value) => {
                        let (low, high) = unsafe { transmute::<u32, (u16, u16)>(value) };
                        s.set_general_register_value(StarGeneralRegister::Aux2, low);
                        s.set_general_register_value(StarGeneralRegister::Aux3, high);
                    }
                    Err(_) => return true,
                }
            }
            13 => {
                // read character
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let trimmed = input.trim();
                if trimmed.len() == 1 {
                    let c = trimmed.chars().next().unwrap() as u16;
                    s.set_general_register_value(StarGeneralRegister::Aux2, c);
                } else {
                    return true;
                }
            }
            14 => {
                // read string with maximum length
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                let mut written: usize = 0;
                for c in input.chars() {
                    if written >= registers.aux3 as usize {
                        break;
                    }
                    let address = match registers.aux2.checked_add(written as u16) {
                        Some(a) => a,
                        None => return true,
                    };
                    if s.get_data_memory_mut().store(address, c as u8).is_err() {
                        return true;
                    }
                    written += 1;
                }

                let len = match u16::try_from(written) {
                    Ok(v) => v,
                    Err(_) => return true,
                };
                s.set_general_register_value(StarGeneralRegister::Aux2, len);
            }
            15 => {
                // read zero terminated string with maximum length
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                let max_len = registers.aux3 as usize;
                if max_len > 0 {
                    let mut i = 0usize;
                    for c in input.chars() {
                        if i + 1 >= max_len {
                            break;
                        }
                        let address = match registers.aux2.checked_add(i as u16) {
                            Some(a) => a,
                            None => return true,
                        };
                        if s.get_data_memory_mut().store(address, c as u8).is_err() {
                            return true;
                        }
                        i += 1;
                    }
                    let zaddr = match registers.aux2.checked_add(i as u16) {
                        Some(a) => a,
                        None => return true,
                    };
                    if s.get_data_memory_mut().store(zaddr, 0).is_err() {
                        return true;
                    }
                }
            }
            16 => return true,
            17 => {
                // print instruction
                let idx = (registers.aux2 as usize) * 2;
                let high = *s.get_instruction_memory().get(idx).ok_or(true).unwrap();
                let low = *s.get_instruction_memory().get(idx + 1).ok_or(true).unwrap();
                let instr = unsafe { transmute::<(u8, u8), u16>((low, high)) };
                print!("\n{:016b} ", instr);
                io::stdout().flush().unwrap();
            }
            18 => {
                // sleep
                std::thread::sleep(std::time::Duration::from_millis(registers.aux2 as u64));
            }
            19 => {
                // random u16
                let value = rand::random::<u16>();
                s.set_general_register_value(StarGeneralRegister::Aux2, value);
            }
            _ => {}
        }

        false
    }
}