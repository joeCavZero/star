use std::fs::File;
use std::io::Write;

use crate::cli::*;
use crate::debugger;
use crate::star::binaryable::Binaryable;
use crate::star::executable::Executable;
use crate::star::resolveable::SymbolTable;
use crate::star::Star;

pub trait Runnable {
    fn run(&mut self);
    
    fn generate_binary_file(&self, destiny: String, star: &Star, data_section_size: usize);
}

impl Runnable for Cli {
    fn run(&mut self) {

        if self.version {
            debugger::info_message("Star version 0.1.0");
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
        if let Some(file) = self.file.clone() {
            let (symbol_table, data_section_count) = star.process_from_file(&file);
            if let Some(bin_dest) = self.binary_destiny.clone() {
                self.generate_binary_file(bin_dest, &star, data_section_count);
            } else {
                star.execute();

                if self.symbol_table {
                    display_symbol_table(&symbol_table);
                }

                if self.registers {
                    display_registers(&star);
                }
            }
        } 
        if let Some(from_bin) = self.from_binary.clone() {
            star.process_from_binary(&from_bin);
            star.execute();
            debugger::new_line();
            if self.registers {
                display_registers(&star);
            }
        }
        
    }

    

    fn generate_binary_file(&self, destiny: String, star: &Star, data_section_size: usize) {
        match File::create(&destiny) {
            Ok(mut file) => {
                match file.write(".instr".as_bytes()) {
                    Ok(_) => {}
                    Err(_) => debugger::exit_with_error("Failed to write on binary file"),
                }

                for (index, instr) in star.instruction_memory.iter().enumerate() {
                    let mut output = format!("{:08b} ", instr);
                    if index % 2 == 0 {
                        output = format!("\n{}", output);
                    }
                    match file.write(output.as_bytes()) {
                        Ok(_) => {}
                        Err(_) => debugger::exit_with_error("Failed to write on binary file"),
                    }
                }

                match file.write("\n.data".as_bytes()) {
                    Ok(_) => {}
                    Err(_) => debugger::exit_with_error("Failed to write on binary file"),
                }

                for byte_index in 0..data_section_size {
                    let byte = match star.data_memory.get(byte_index) {
                        Some(b) => *b,
                        None => {
                            debugger::exit_with_error("Data memory out of bounds");
                            unreachable!();
                        }
                    };

                    let output = format!("\n{:08b} ", byte);
                    match file.write(output.as_bytes()) {
                        Ok(_) => {}
                        Err(_) => debugger::exit_with_error("Failed to write on binary file"),
                    }
                }
            }
            Err(_) => debugger::exit_with_error("Failed to create binary file"),
        }
    }
}


fn display_registers(star: &Star) {
    debugger::message("================== Registers ==================");
    debugger::message(&format!("zero ---------------> [0b{:016b}] [{}]", star.registers.zero, star.registers.zero));
    debugger::message(&format!("a ------------------> [0b{:016b}] [{}]", star.registers.a, star.registers.a));
    debugger::message(&format!("b ------------------> [0b{:016b}] [{}]", star.registers.b, star.registers.b));
    debugger::message(&format!("c ------------------> [0b{:016b}] [{}]", star.registers.c, star.registers.c));
    debugger::message(&format!("d ------------------> [0b{:016b}] [{}]", star.registers.d, star.registers.d));
    debugger::message(&format!("e ------------------> [0b{:016b}] [{}]", star.registers.e, star.registers.e));
    debugger::message(&format!("f ------------------> [0b{:016b}] [{}]", star.registers.f, star.registers.f));
    debugger::message(&format!("g ------------------> [0b{:016b}] [{}]", star.registers.g, star.registers.g));
    debugger::message(&format!("aux1 ---------------> [0b{:016b}] [{}]", star.registers.aux1, star.registers.aux1));
    debugger::message(&format!("aux2 ---------------> [0b{:016b}] [{}]", star.registers.aux2, star.registers.aux2));
    debugger::message(&format!("aux3 ---------------> [0b{:016b}] [{}]", star.registers.aux3, star.registers.aux3));
    debugger::message(&format!("carry --------------> [0b{:016b}] [{}]", star.registers.carry, star.registers.carry));
    debugger::message(&format!("low ----------------> [0b{:016b}] [{}]", star.registers.low, star.registers.low));
    debugger::message(&format!("high ---------------> [0b{:016b}] [{}]", star.registers.high, star.registers.high));
    debugger::message(&format!("return address -----> [0b{:016b}] [{}]", star.registers.return_address, star.registers.return_address));
    debugger::message(&format!("stack pointer ------> [0b{:016b}] [{}]", star.registers.stack_pointer, star.registers.stack_pointer));
    debugger::message(&format!("program counter ----> [0b{:016b}] [{}]", star.registers.program_counter, star.registers.program_counter));
    println!();
}

fn display_symbol_table(symbol_table: &SymbolTable) {
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