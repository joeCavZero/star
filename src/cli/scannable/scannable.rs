use crate::cli::*;
use crate::debugger;

pub trait Scannable {
    fn scan(&mut self);
}

impl Scannable for Cli {
    fn scan(&mut self) {
        let args = std::env::args().collect::<Vec<String>>();
        if args.len() < 2 {
            self.help = true;
            return;
        }
        
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
                            self.file = Some(file.clone());
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
                "-fb"
                | "--from-binary"
                    => {
                        if self.from_binary.is_some() {
                            debugger::exit_with_error("Cannot specify more than one from binary option");
                        }
                        match args.get(arg_counter + 1) {
                            Some(file) => {
                                self.from_binary = Some(file.clone());
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
                "-v"
                | "--version"   
                    => self.version = true,
                
                "-h"
                | "--help" 
                    => self.help = true,
                
                "-st"
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
                    if self.file.is_some() {
                        debugger::exit_with_error("Cannot specify more than one base file");
                    }
                    self.file = Some(arg.clone());
                }
            }
            arg_counter += 1;
        }
        
        let mut incompatible_args = false;

        if self.version && self.help {
            incompatible_args = true;
        }

        if (self.version || self.help)
            && (self.file.is_some()
            || self.binary_destiny.is_some()
            || self.symbol_table
            || self.registers
            || self.from_binary.is_some())
        {
            incompatible_args = true;
        }

        if self.file.is_some() && self.from_binary.is_some() {
            incompatible_args = true;
        }

        if self.file.is_some()
            && self.binary_destiny.is_some()
            && self.from_binary.is_some()
        {
            incompatible_args = true;
        }

        if self.from_binary.is_some()
            && (self.symbol_table || self.binary_destiny.is_some())
        {
            incompatible_args = true;
        }

        if self.from_binary.is_some() && self.symbol_table {
            incompatible_args = true;
        }

        if incompatible_args {
            self.version = false;
            self.file = None;
            self.binary_destiny = None;
            self.symbol_table = false;
            self.registers = false;
            self.from_binary = None;

            self.help = true;

            debugger::message("Incorrect usage of options");
        }

            
        
    }
}