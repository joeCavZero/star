use crate::cli::*;
use crate::debugger;

pub trait Scanneable {
    fn scan(&mut self);
}

impl Scanneable for Cli {
    fn scan(&mut self) {
        let args = std::env::args().collect::<Vec<String>>();
        let mut arg_counter = 1;
        while arg_counter < args.len() {
            let arg = match args.get(arg_counter) {
                Some(arg) => arg,
                None => break,
            };
            
            match arg.as_str() {
                "-f" 
                | "--file"
                => {
                    match args.get(arg_counter + 1) {
                        Some(file) => {
                            self.base_file = Some(file.clone());
                            arg_counter += 1;
                        }
                        None => {
                            debugger::exit_with_error(
                                &format!("Expected a file path after '{}'", arg)
                            );
                            unreachable!();
                        }
                    }
                }
                
                "-b" 
                | "--binary"
                => {
                    if self.binary_destiny.is_some() {
                        debugger::exit_with_error("Cannot specify more than one binary destination");
                    }
                    match args.get(arg_counter + 1) {
                        Some(file) => {
                            self.binary_destiny = Some(file.clone());
                            arg_counter += 1;
                        }
                        None => {
                            debugger::exit_with_error(
                                &format!("Expected a binary destination after '{}'", arg)
                            );
                            unreachable!();
                        }
                    }
                }
                
                "-v"
                | "--version"   
                    => self.version = true,
                
                "-h"
                | "--help" 
                    => self.help = true,
                
                "-s"
                | "--symbol-table" 
                    => self.symbol_table = true,
                
                "-r"
                | "--registers" 
                    => self.registers = true,

                _ if arg.starts_with("-") 
                    => debugger::exit_with_error(&format!("Unknown option '{}'", arg)),
                
                _ if arg.starts_with("--")
                    => debugger::exit_with_error(&format!("Unknown long option '{}'", arg)),
                
                _ => {
                    if self.base_file.is_some() {
                        debugger::exit_with_error("Cannot specify more than one base file");
                    }
                    self.base_file = Some(arg.clone());
                }
            }
            arg_counter += 1;
        }

        if (self.version || self.help)
        && (
            self.base_file.is_some()
            || self.binary_destiny.is_some()
            || self.symbol_table
            || self.registers
        ) {
            debugger::exit_with_error("Cannot use version and help with other options");
        }

        // ==== CHECKING FOR INCONSISTENCIES ====
        
        if !self.version && !self.help {
            // check if base file is specified
            if self.base_file.is_none() {
                debugger::exit_with_error("No base file specified");
            }

            // check if binary destination is specified with symbol table or registers mode
            if 
            self.binary_destiny.is_some() 
            && (
                self.symbol_table 
                || self.registers
            ) {
                debugger::exit_with_error("Cannot use binary destination with symbol table or registers");
            }
        }
    }
}