use std::collections::HashMap;
use crate::star::utils::*;
use crate::star::core::*;
use crate::star::generateable::*;
use crate::star::resolveable::*;
use crate::star::scannable::*;
use crate::star::parseable::*;

pub const DATA_MEMORY_SIZE: usize = 65536;

pub struct Star {
    pub file_table: HashMap<u32, String>,
    pub data_memory: [u8; DATA_MEMORY_SIZE],
    pub instruction_memory: Vec<u8>,
    pub position_memory: Vec<Position>,
    pub registers: Registers,
}

impl Star {
    pub fn new() -> Self {
        let mut data_memory = [0; DATA_MEMORY_SIZE];
        for b in data_memory.iter_mut() {
            *b = rand::random::<u8>();
        }
        Self {
            file_table: HashMap::new(),
            data_memory: data_memory,
            instruction_memory: Vec::new(),
            position_memory: Vec::new(),
            registers: Registers::new(),
        }
    }

    pub fn process_from_file(&mut self, file_path: &String) -> (SymbolTable, usize) {
        let ptokens = self.scan(file_path);
        let mut ast = self.parse(&ptokens);
        let symbol_table = self.resolve(&mut ast);
        let data_section_size = self.generate(&ast);
        return (symbol_table, data_section_size);
    }

    pub fn get_file_id_by_path(&self, file_path: &String) -> Option<u32> {
        self.file_table.iter().find_map(|(id, path)| if path == file_path { Some(*id) } else { None })
    }

    pub fn get_file_name(&self, file_id: u32) -> String {
        match self.file_table.get(&file_id) {
            Some(name) => name.clone(),
            None => "Unknown file".to_string(),
        }
    }
}

