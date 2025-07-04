use std::collections::HashMap;
use crate::star::scanneable::*;

const DATA_MEMORY_SIZE: usize = 65536;

pub struct Star {
    pub file_table: HashMap<u32, String>,
    pub data_memory: [u8; DATA_MEMORY_SIZE],
}

impl Star {
    pub fn new() -> Self {
        Self {
            file_table: HashMap::new(),
            data_memory: [0; DATA_MEMORY_SIZE],
        }
    }

    pub fn load(&mut self, base_file_path: &String) {
        let ptokens = self.scan(base_file_path);
        println!("{:#?}", ptokens);
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