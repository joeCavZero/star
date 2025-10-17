use std::mem::transmute;
use std::io::{self, Write};

use crate::star::Star;
use crate::star::math::*;
use crate::star::executable::Executable;
use crate::star::debuggable::Debugable;

pub fn default(star: &mut Star) -> bool {
    let instruction_position_option = star
                .position_memory
                .get(star.registers.program_counter as usize)
                .cloned();
    match star.registers.aux1 {
        1 => {
            // print unsigned byte
            let low: u8 = unsafe {
                transmute::<u16, (u8, u8)>(star.registers.aux2).0
            };
            print!("{}", low);
            io::stdout().flush().unwrap();
        }
        2 => {
            // print signed byte
            let low: u8 = unsafe {
                transmute::<u16, (u8, u8)>(star.registers.aux2).0
            };
            let v: i8 = u8::cast_signed(low);
            print!("{}", v);
            io::stdout().flush().unwrap();
        }
        3 => {
            // print unsigned word
            print!("{}", star.registers.aux2);
            io::stdout().flush().unwrap();
        }
        4 => {
            // print signed word
            let v: i16 =
                u16::cast_signed(star.registers.aux2)
            ;
            print!("{}", v);
            io::stdout().flush().unwrap();
        }
        5 => {
            // print unsigned double
            let low: u16 = star.registers.aux2;
            let high: u16 = star.registers.aux3;
            let value: u32 = unsafe {
                transmute::<(u16, u16), u32>((low, high))
            };
            print!("{}", value);
            io::stdout().flush().unwrap();
        }
        6 => {
            // print signed double
            let low: u16 = star.registers.aux2;
            let high: u16 = star.registers.aux3;
            let value: i32 = unsafe {
                transmute::<(u16, u16), i32>((low, high))
            };
            print!("{}", value);
            io::stdout().flush().unwrap();
        }
        7 => {
            // print char
            let low: u8 = unsafe {
                transmute::<u16, (u8, u8)>(star.registers.aux2).0
            };

            let c = low as char;
            print!("{}", c);
            io::stdout().flush().unwrap();
        }
        8 => {
            // print string with lenght (\0 not effects the string to print)
            let base_address = star.registers.aux2;
            let length = star.registers.aux3;

            let mut string: String = String::new();

            for i in 0..length {
                match base_address.checked_add(i) {
                    Some(address) => {
                        match star.load_from_data_memory(address) {
                    Ok(v) => {
                        string.push(v as char);
                    }
                    Err(_) => star.exit_with_optional_positional_error(
                        "String length exceeds memory bounds",
                        instruction_position_option,
                    ),
                }
                    }
                    None => star
                        .exit_with_optional_positional_error(
                            "String length exceeds memory bounds",
                            instruction_position_option,
                        ),
                }
            }

            print!("{}", string);
            io::stdout().flush().unwrap();
        }
        9 => {
            // print zero terminated string
            let base_address = star.registers.aux2;

            let mut string: String = String::new();
            let mut i = 0;
            loop {
                match base_address.checked_add(i) {
                    Some(address) => {
                        match star.load_from_data_memory(address) {
                    Ok(v) => {
                        if v == 0 {
                            break;
                        }
                        string.push(v as char);
                    }
                    Err(_) => star.exit_with_optional_positional_error(
                        "String length exceeds memory bounds",
                        instruction_position_option,
                    ),
                }
                    }
                    None => star
                        .exit_with_optional_positional_error(
                            "String length exceeds memory bounds",
                            instruction_position_option,
                        ),
                }
                i += 1;
            }

            print!("{}", string);
            io::stdout().flush().unwrap();
        }
        10 => {
            // read byte
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match u8_from_string(input.trim().to_string()) {
                Ok(value) => {
                    star.registers.aux2 = unsafe {
                        transmute::<(u8, u8), u16>((value, 0))
                    };
                }
                Err(_) => {
                    star.exit_with_optional_positional_error(
                        "Invalid input for byte",
                        instruction_position_option,
                    );
                }
            }
        }
        11 => {
            // read word
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match u16_from_string(input.trim().to_string()) {
                Ok(value) => {
                    star.registers.aux2 = value;
                }
                Err(_) => {
                    star.exit_with_optional_positional_error(
                        "Invalid input for 16 bit word",
                        instruction_position_option,
                    );
                }
            }
        }
        12 => {
            // read double
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match u32_from_string(input.trim().to_string()) {
                Ok(value) => {
                    let (low, high) = unsafe {
                        transmute::<u32, (u16, u16)>(value)
                    };
                    star.registers.aux2 = low;
                    star.registers.aux3 = high;
                }
                Err(_) => {
                    star.exit_with_optional_positional_error(
                        "Invalid input for 32 bit double",
                        instruction_position_option,
                    );
                }
            }
        }
        13 => {
            // read character
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed = input.trim();
            if trimmed.len() == 1 {
                let c = trimmed.chars().next().unwrap();
                star.registers.aux2 = c as u16;
            } else {
                star.exit_with_optional_positional_error(
            "Invalid input for character, it must be a single character",
            instruction_position_option,
        );
            }
        }
        14 => {
            // read string with a maximum length
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            let base_address_to_store = star.registers.aux2;
            let max_address_to_reach = match base_address_to_store
                .checked_add(star.registers.aux3)
            {
                Some(addr) => addr,
                None => {
                    star.exit_with_optional_positional_error(
                        "String length exceeds memory bounds",
                        instruction_position_option,
                    );
                    unreachable!();
                }
            };

            let mut i = 0;
            for c in input.chars() {
                if i >= star.registers.aux3 as usize {
                    break;
                }
                match base_address_to_store.checked_add(i as u16) {
                    Some(address) => {
                        if address >= max_address_to_reach {
                            break;
                        }
                        match star.store_on_data_memory(address, c as u8) {
                    Ok(_) => {}
                    Err(_) => star.exit_with_optional_positional_error(
                        "String exceeds memory bounds",
                        instruction_position_option,
                    ),
                }
                    }
                    None => star
                        .exit_with_optional_positional_error(
                            "String exceeds memory bounds",
                            instruction_position_option,
                        ),
                }
                i += 1;
            }

            // sets the aux2 register to the length of the string inserted
            star.registers.aux2 = match u16::try_from(input.len()) {
                Ok(len) => len,
                Err(_) => {
                    star.exit_with_optional_positional_error(
                "String length exceeds maximum size of 16 bits",
                instruction_position_option,
            );
                    unreachable!();
                }
            };
        }
        15 => {
            // read string zero with a maximum length, it always put a \0 at the end of the string inserted on memory
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
            let base_address_to_store = star.registers.aux2;
            let max_len = star.registers.aux3 as usize;

            if max_len == 0 {
                // Nothing LOL
            } else {
                let mut i = 0;
                for c in input.chars() {
                    if i + 1 >= max_len {
                        break;
                    }
                    match base_address_to_store
                        .checked_add(i as u16)
                    {
                        Some(address) => {
                            match star.store_on_data_memory(address, c as u8) {
                        Ok(_) => {}
                        Err(_) => star.exit_with_optional_positional_error(
                            "String exceeds memory bounds",
                            instruction_position_option,
                        ),
                    }
                        }
                        None => star
                            .exit_with_optional_positional_error(
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
                            match star.store_on_data_memory(address, 0) {
                        Ok(_) => {}
                        Err(_) => star.exit_with_optional_positional_error(
                            "String exceeds memory bounds",
                            instruction_position_option,
                        ),
                    }
                        }
                    }
                    None => star
                        .exit_with_optional_positional_error(
                            "String exceeds memory bounds",
                            instruction_position_option,
                        ),
                }
            }
        }
        16 => {
            return true;
        }
        17 => {
            // print instruction
            let target_instr_index: usize =
                (star.registers.aux2 as usize) * 2;
            let (target_instr_high, target_instr_low) = (
                match star
                    .instruction_memory
                    .get(target_instr_index)
                {
                    Some(byte) => *byte,
                    None => {
                        star.exit_with_optional_positional_error(
                            "Target instruction out of bounds",
                            instruction_position_option,
                        );
                        unreachable!();
                    }
                },
                match star
                    .instruction_memory
                    .get(target_instr_index + 1)
                {
                    Some(byte) => *byte,
                    None => {
                        star.exit_with_optional_positional_error(
                            "Target instruction out of bounds",
                            instruction_position_option,
                        );
                        unreachable!();
                    }
                },
            );

            let target_instruction_format = unsafe {
                transmute::<(u8, u8), u16>((
                    target_instr_low,
                    target_instr_high,
                ))
            };

            print!("\n{:016b} ", target_instruction_format);
            io::stdout().flush().unwrap();
        }
        18 => {
            // sleep
            let millis: u64 = star.registers.aux2 as u64;
            std::thread::sleep(std::time::Duration::from_millis(
                millis,
            ));
        }
        19 => {
            // random u16 number
            let random_value= rand::random::<u16>();
            star.registers.aux2 = random_value;
        }
        _ => {}
    }
    
    return false;                           
}