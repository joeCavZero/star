use std::io::{self, Write};

use star::prelude::*;
use star::resolveable::StarSymbolTable; // Star, StarPosition, etc.

use crate::debugger;
use crate::interface::Interface;

#[derive(Debug)]
pub struct Cli {
    pub version: bool,
    pub help: bool,
    pub file: Option<String>,
    pub binary_destiny: Option<String>,
    pub from_binary: Option<String>,
    pub symbol_table: bool,
    pub registers: bool,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            version: false,
            help: false,
            file: None,
            binary_destiny: None,
            from_binary: None,
            symbol_table: false,
            registers: false,
        }
    }

    pub fn run(&mut self) {
        if self.version {
            debugger::info_message("Star version 0.2.0");
            return;
        }

        if self.help {
            debugger::message("Usage: star [FILE] [OPTIONS]");
            debugger::message("Options:");
            debugger::message("  -v, --version               Show version information");
            debugger::message("  -h, --help                  Show this help message");
            debugger::message("  -f, --file <FILE>           Specify the base file to run");
            debugger::message("  -b, --binary <FILE>         Specify the binary destination file");
            debugger::message("  -fb, --from-binary <FILE>   Specify the binary file to run");
            debugger::message("  -r, --registers             Display the registers");
            debugger::message("  -st, --symbol-table         Display the symbol table");
            return;
        }

        let mut star = Star::new();

        star.set_interface(Box::new(Interface::new()));

        if let Some(file) = self.file.clone() {
            // 1) Load
            let load_res = star.load_from_assembly_file(&file);

            match load_res {
                Ok((symbol_table, data_section_size)) => {
                    if let Some(bin_dest) = self.binary_destiny.clone() {
                        if let Err(err) =
                            star.save_loaded_binary(bin_dest.as_str(), data_section_size)
                        {
                            debugger::exit_with_error(err.as_str());
                        }
                        return;
                    }

                    match star.execute() {
                        Ok(_) => {
                            println!();
                            io::stdout().flush().unwrap();
                        }
                        Err((err, pos)) => {
                            debugger::exit_with_optional_positional_error(&star, &err, pos);
                            return;
                        }
                    }

                    if self.symbol_table {
                        display_symbol_table(&symbol_table);
                    }

                    if self.registers {
                        display_registers(&star);
                    }
                }
                Err((err, pos)) => {
                    debugger::exit_with_optional_positional_error(&star, &err, pos);
                    return;
                }
            }
        }

        if let Some(from_bin) = self.from_binary.clone() {
            match star.load_from_binary_file(&from_bin) {
                Ok(_) => {
                    match star.execute() {
                        Ok(_) => {
                            debugger::new_line();
                            if self.registers {
                                display_registers(&star);
                            }
                        }
                        Err((err, pos)) => {
                            debugger::exit_with_optional_positional_error(&star, &err, pos);
                            return;
                        }
                    }
                }
                Err((err, pos)) => {
                    debugger::exit_with_optional_positional_error(&star, &err, pos);
                    return;
                }
            }
        }
    }

}





fn display_registers(star: &Star) {
    debugger::message("================== Registers ==================");
    debugger::message(&format!("zero ----------------> [0b{:016b}] [{}]", star.registers.zero, star.registers.zero));
    debugger::message(&format!("a -------------------> [0b{:016b}] [{}]", star.registers.a, star.registers.a));
    debugger::message(&format!("b -------------------> [0b{:016b}] [{}]", star.registers.b, star.registers.b));
    debugger::message(&format!("c -------------------> [0b{:016b}] [{}]", star.registers.c, star.registers.c));
    debugger::message(&format!("d -------------------> [0b{:016b}] [{}]", star.registers.d, star.registers.d));
    debugger::message(&format!("e -------------------> [0b{:016b}] [{}]", star.registers.e, star.registers.e));
    debugger::message(&format!("f -------------------> [0b{:016b}] [{}]", star.registers.f, star.registers.f));
    debugger::message(&format!("g -------------------> [0b{:016b}] [{}]", star.registers.g, star.registers.g));
    debugger::message(&format!("aux1 ----------------> [0b{:016b}] [{}]", star.registers.aux1, star.registers.aux1));
    debugger::message(&format!("aux2 ----------------> [0b{:016b}] [{}]", star.registers.aux2, star.registers.aux2));
    debugger::message(&format!("aux3 ----------------> [0b{:016b}] [{}]", star.registers.aux3, star.registers.aux3));
    debugger::message(&format!("carry ---------------> [0b{:016b}] [{}]", star.registers.carry, star.registers.carry));
    debugger::message(&format!("low -----------------> [0b{:016b}] [{}]", star.registers.low, star.registers.low));
    debugger::message(&format!("high ----------------> [0b{:016b}] [{}]", star.registers.high, star.registers.high));
    debugger::message(&format!("return address ------> [0b{:016b}] [{}]", star.registers.return_address, star.registers.return_address));
    debugger::message(&format!("stack pointer -------> [0b{:016b}] [{}]", star.registers.stack_pointer, star.registers.stack_pointer));
    debugger::message(&format!("program counter -----> [0b{:016b}] [{}]", star.registers.program_counter, star.registers.program_counter));
    debugger::message(&format!("instruction pointer--> [0b{:016b}] [{}]", star.registers.instruction_pointer, star.registers.instruction_pointer));
    debugger::message(&format!("instruction register-> [0b{:016b}] [{}]", star.registers.instruction_register, star.registers.instruction_register));
    println!();
}

fn display_symbol_table(symbol_table: &StarSymbolTable) {
    debugger::message("================ Symbol Table =================");
    for (name, address) in symbol_table.iter() {
        let name_str = format!("[{}]", name);
        let address_str = format!("[0x{:04X}] [{}]", address, address);
        
        let mut arrow_length: usize = 19;
        arrow_length = arrow_length.saturating_sub(name_str.len());
        let mut arrow = "-".repeat(arrow_length);
        arrow.push('>');
        debugger::message(&format!("{} {} {}", name_str, arrow, address_str));
    }
    println!();
}