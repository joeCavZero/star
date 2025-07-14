use std::collections::HashMap;

use crate::star::core::*;
use crate::star::debuggable::*;
use crate::star::math::*;
use crate::star::parseable::*;
use crate::star::resolveable::*;
use crate::star::utils::*;

pub trait Symbolable {
    fn get_symbol_table(&self, ast: &mut Ast) -> SymbolTable;
}

impl Symbolable for Star {
    fn get_symbol_table(&self, ast: &mut Ast) -> SymbolTable {
        let mut symbol_table: SymbolTable = HashMap::new();
        // >>>> DATA MEMORY <<<<
        let mut data_memory_counter: usize = 0;
        for data_camp in ast.data_field.iter() {
            // ==== Process of collecting labels ====
            for label_ptk in data_camp.label_declarations.iter() {
                match label_ptk.token {
                    Token::LabelDeclaration(ref label_name) => {
                        if symbol_table.contains_key(label_name) {
                            self.exit_with_positional_error(
                                format!(
                                    "Label '{}' already declared",
                                    label_name,
                                ).as_str(),
                                label_ptk.position,
                            );
                        } else {
                            match u16::try_from(data_memory_counter) {
                                Ok(address) => {
                                    symbol_table.insert(label_name.clone(), address);
                                }
                                Err(_) => {
                                    self.exit_with_positional_error(
                                        format!(
                                            "Label '{}' address exceeds 16 bits",
                                            label_name,
                                        ).as_str(),
                                        label_ptk.position,
                                    );
                                }
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }

            // ==== Process of moving the data memory counter ====
            match data_camp.directive.token {
                // ==== BYTE DIRECTIVE ====
                Token::Directive(Directive::Byte) => {
                    if let DataCampArg::Multiple(ref ptk_args) = data_camp.arg {
                        for ptk_arg in ptk_args.iter() {
                            match data_memory_counter.checked_add(1) {
                                Some(new_value) => {
                                    if new_value > DATA_MEMORY_SIZE {
                                        self.exit_with_positional_error(
                                            "Data memory overflow",
                                            ptk_arg.position,
                                        );
                                    } else {
                                        data_memory_counter = new_value;
                                    }
                                }
                                None => self.exit_with_positional_error(
                                    "Data memory overflow",
                                    ptk_arg.position,
                                ),
                            }
                        }
                    } else {
                        unreachable!();
                    }
                }
                // ==== WORD DIRECTIVE ====
                Token::Directive(Directive::Word) => {
                    if let DataCampArg::Multiple(ref ptk_args) = data_camp.arg {
                        for ptk_arg in ptk_args.iter() {
                            match data_memory_counter.checked_add(2) {
                                Some(new_value) => {
                                    if new_value > DATA_MEMORY_SIZE {
                                        self.exit_with_positional_error(
                                            "Data memory overflow",
                                            ptk_arg.position,
                                        );
                                    } else {
                                        data_memory_counter = new_value;
                                    }
                                }
                                None => self.exit_with_positional_error(
                                    "Data memory overflow",
                                    ptk_arg.position,
                                ),
                            }
                        }
                    } else {
                        unreachable!();
                    }
                }

                // ==== SPACE DIRECTIVE ====
                Token::Directive(Directive::Space) => {
                    if let DataCampArg::Unique(ref ptk_arg) = data_camp.arg {
                        match ptk_arg.token {
                            Token::NumberLiteral(ref num_string) => {
                                match u16_from_string(num_string.clone()) {
                                    Ok(num) => {
                                        match data_memory_counter.checked_add(num as usize) {
                                            Some(ndmv) => {
                                                if ndmv > DATA_MEMORY_SIZE {
                                                    self.exit_with_positional_error(
                                                        "Data memory overflow",
                                                        ptk_arg.position,
                                                    );
                                                } else {
                                                    data_memory_counter = ndmv;
                                                }
                                            }
                                            None => {
                                                self.exit_with_positional_error(
                                                    "Data memory overflow",
                                                    ptk_arg.position,
                                                );
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        self.exit_with_positional_error(
                                            err.as_str(),
                                            ptk_arg.position,
                                        );
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        unreachable!();
                    }
                }

                // ==== STRING AND STRINGZ DIRECTIVES ====
                Token::Directive(Directive::String)
                | Token::Directive(Directive::Stringz) => {
                    if let DataCampArg::Unique(ref ptk_arg) = data_camp.arg {
                        match ptk_arg.token {
                            Token::StringLiteral(ref string_literal) => {
                                let mut string_len = string_literal.len();
                                if data_camp.directive.token == Token::Directive(Directive::Stringz) {
                                    string_len += 1; // null terminator add
                                }
                                match u16::try_from(string_len) {
                                    Ok(len) => {
                                        match data_memory_counter.checked_add(len as usize) {
                                            Some(new_value) => {
                                                if new_value > DATA_MEMORY_SIZE {
                                                    self.exit_with_positional_error(
                                                        "Data memory overflow",
                                                        ptk_arg.position,
                                                    );
                                                } else {
                                                    data_memory_counter = new_value;
                                                }
                                            }
                                            None => self.exit_with_positional_error(
                                                "Data memory overflow",
                                                ptk_arg.position,
                                            ),
                                        }
                                    }
                                    Err(_) => {
                                        self.exit_with_positional_error(
                                            "String length exceeds 16 bits",
                                            ptk_arg.position,
                                        );
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        unreachable!();
                    }
                }
                Token::Directive(Directive::Checkpoint) => {
                    if let DataCampArg::Empty = data_camp.arg {
                        // Nothing to do here
                    } else {
                        unreachable!();
                    }
                }
                _ => unreachable!(),
            }
        }

        // >>>> INSTRUCTION MEMORY <<<<
        for (instruction_index, instruction) in ast.instr_field.iter().enumerate() {
            for label_ptk in instruction.label_declarations.iter() {
                match label_ptk.token {
                    Token::LabelDeclaration(ref label_name) => {
                        if symbol_table.contains_key(label_name) {
                            self.exit_with_positional_error(
                                format!(
                                    "Label '{}' already declared",
                                    label_name,
                                ).as_str(),
                                label_ptk.position,
                            );
                        } else {
                            match u16::try_from(instruction_index) {
                                Ok(address) => {
                                    symbol_table.insert(label_name.clone(), address);
                                }
                                Err(_) => {
                                    self.exit_with_positional_error(
                                        format!(
                                            "Label '{}' address exceeds 16 bits",
                                            label_name,
                                        ).as_str(),
                                        label_ptk.position,
                                    );
                                }
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        symbol_table
    }
}