use std::collections::HashMap;
use crate::star::core::*;
use crate::star::executable::Executable;
use crate::star::generateable::*;
use crate::star::resolveable::*;
use crate::star::scanneable::*;
use crate::star::parseable::*;

pub const DATA_MEMORY_SIZE: usize = 65536;

pub struct Star {
    pub file_table: HashMap<u32, String>,
    pub data_memory: [u8; DATA_MEMORY_SIZE],
    pub instruction_memory: Vec<PositionedInstruction>,
    pub registers: Registers,
}

impl Star {
    pub fn new() -> Self {
        Self {
            file_table: HashMap::new(),
            data_memory: [0; DATA_MEMORY_SIZE],
            instruction_memory: Vec::new(),
            registers: Registers::new(),
        }
    }

    pub fn init(&mut self, base_file_path: &String) {
        self.process(base_file_path);
        self.execute();
        //println!("{:#?}", self.registers);
        //println!("Data Memory (first 20 bytes): {:?}", &self.data_memory[..20]);
        //println!("{:#?}", ast);
        //for instr in self.instruction_memory.iter() {
        //    println!("{:016b}", instr.format);
        //}
    }

    pub fn process(&mut self, file_path: &String) {
        let ptokens = self.scan(file_path);
        let mut ast = self.parse(&ptokens);
        self.resolve(&mut ast);
        self.generate(&ast);
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

