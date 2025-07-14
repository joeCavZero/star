use crate::{debugger, star::{debuggable::Debugable, math::u8_from_string, utils::Position, Star}};

/*
    The main idea of binaryable is to provide a way to process binary files
    that contain instructions and data for the Star virtual machine.

    The process of reading a binary file is simpler than reading a source
    code file, as it does not require parsing or resolving symbols.
    Instead, it directly reads the binary data and stores it in the instruction
    and data memory of the Star virtual machine.

    Not very complicated thing should be done here, just reading and
    inserting the data and instructions into the memory.
    It is the main reason why this code is so simple.
*/

pub trait Binaryable {
    fn process_from_binary(&mut self, file_path: &String);
}

impl Binaryable for Star {
    fn process_from_binary(&mut self, file_path: &String) {
        use std::fs;
        let absolute_file_path: String = match fs::canonicalize(file_path) {
            Ok(path) => path.to_str().unwrap_or(file_path).to_string(),
            Err(_) => {
                debugger::exit_with_error(&format!("Failed to read binary file '{}'", file_path));
                unreachable!();
            }
        };

        self.file_table.insert(self.file_table.len() as u32, absolute_file_path.clone());
        let id_option = self.get_file_id_by_path(&absolute_file_path);
        if id_option.is_none() {
            debugger::exit_with_error(&format!("File '{}' not found in file table", absolute_file_path));
            return;
        }
        let id = id_option.unwrap();

        let file_content = match fs::read_to_string(&absolute_file_path) {
            Ok(content) => content.replace("\r", ""),
            Err(_) => {
                debugger::exit_with_error(&format!("Failed to read binary file '{}'", absolute_file_path));
                unreachable!();
            }
        };

        let mut section = ".instr".to_string();

        let mut actual_line = 1;
        let mut actual_column = 1;

        let mut data_memory_vector: Vec<u8> = Vec::new();

        let mut tkn_line = 1;
        let mut tkn_column = 1;
        
        let mut line_has_identation = false;

        let mut accumulator = String::new();
        let mut chars = file_content.chars().peekable();
        while let Some(ch) = chars.next() {
            match ch {
                '\n' 
                | '\t'
                | ' '
                => {
                    if ch == '\n' {
                        actual_line += 1;
                        line_has_identation = false;
                    } else if ch == '\t' {
                        line_has_identation = true;
                    } else if ch == ' ' {
                        actual_column += 1;
                    }

                    if accumulator == ".instr" {
                        section = ".instr".to_string();
                        accumulator.clear();
                    } else if accumulator == ".data" {
                        section = ".data".to_string();
                        accumulator.clear();
                    } else if !accumulator.is_empty() {
                        match section.as_str() {
                            ".instr" => {
                                let mut aux = "0b".to_string();
                                aux.push_str(&accumulator);
                                
                                match u8_from_string(aux.clone()) {
                                    Ok(v) => {
                                        self.instruction_memory.push(v);
                                    }
                                    Err(_) => {
                                        self.exit_with_positional_error(
                                            "Invalid binary instruction",
                                            Position::new(
                                                id,
                                                tkn_line,
                                                Some(tkn_column),
                                            )
                                        );
                                    }
                                }
                                if self.instruction_memory.len() % 2 == 0 {
                                    self.position_memory.push(
                                        Position::new(
                                            id,
                                            tkn_line,
                                            if line_has_identation { Some(actual_column) } else { None },
                                        )
                                    )
                                }
                            }
                            ".data" => {
                                let mut aux = "0b".to_string();
                                aux.push_str(&accumulator);
                                match u8_from_string(aux) {
                                    Ok(v) => {
                                        data_memory_vector.push(v);
                                    }
                                    Err(_) => {
                                        self.exit_with_positional_error(
                                            "Invalid binary data",
                                            Position::new(
                                                id,
                                                tkn_line,
                                                Some(tkn_column),
                                            )
                                        );
                                    }
                                }
                            }
                            _ => {
                                self.exit_with_positional_error(
                                    "Unknown section",
                                    Position::new(
                                        id,
                                        tkn_line,
                                        Some(tkn_column),
                                    )
                                );
                                unreachable!();
                            }
                        }

                        accumulator.clear();
                    }
                    actual_column += 1;
                }

                _ => {
                    if accumulator.is_empty() {
                        tkn_line = actual_line;
                        tkn_column = actual_column;
                    }
                    accumulator.push(ch);
                    actual_column += 1;
                    continue;
                }
            }
        }
    
        for (i, b) in data_memory_vector.iter().enumerate() {
            match self.data_memory.get_mut(i) {
                Some(memory_byte) => *memory_byte = *b,
                None => {
                    debugger::exit_with_error("Data memory out of bounds");
                    unreachable!();
                }
            }
        }
    }
}