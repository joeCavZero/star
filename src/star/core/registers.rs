use crate::star::{core::DATA_MEMORY_SIZE, utils::GeneralRegister};

#[derive(Debug, Clone)]
pub struct Registers {
    // ==== GENERAL REGISTERS ====
    pub zero: u16,
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub d: u16,
    pub e: u16,
    pub f: u16,
    pub g: u16,
    pub aux1: u16,
    pub aux2: u16,
    pub aux3: u16,
    pub carry: u16,
    pub low: u16,
    pub high: u16,
    pub return_address: u16,
    pub stack_pointer: u16,

    // ==== OCULT REGISTERS ====
    pub program_counter: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            zero: 0,
            a: rand::random::<u16>(),
            b: rand::random::<u16>(),
            c: rand::random::<u16>(),
            d: rand::random::<u16>(),
            e: rand::random::<u16>(),
            f: rand::random::<u16>(),
            g: rand::random::<u16>(),
            aux1: rand::random::<u16>(),
            aux2: rand::random::<u16>(),
            aux3: rand::random::<u16>(),
            carry: rand::random::<u16>(),
            low: rand::random::<u16>(),
            high: rand::random::<u16>(),
            return_address: rand::random::<u16>(),
            stack_pointer: (DATA_MEMORY_SIZE - 1) as u16,
            program_counter: 0,
        }
    }

    pub fn get(&self, register: GeneralRegister) -> u16 {
        match register {
            GeneralRegister::Zero => self.zero,
            GeneralRegister::A => self.a,
            GeneralRegister::B => self.b,
            GeneralRegister::C => self.c,
            GeneralRegister::D => self.d,
            GeneralRegister::E => self.e,
            GeneralRegister::F => self.f,
            GeneralRegister::G => self.g,
            GeneralRegister::Aux1 => self.aux1,
            GeneralRegister::Aux2 => self.aux2,
            GeneralRegister::Aux3 => self.aux3,
            GeneralRegister::Carry => self.carry,
            GeneralRegister::Low => self.low,
            GeneralRegister::High => self.high,
            GeneralRegister::ReturnAddress => self.return_address,
            GeneralRegister::StackPointer => self.stack_pointer,
        }
    }

    pub fn set(&mut self, register: GeneralRegister, value: u16) {
        match register {
            GeneralRegister::Zero => {}
            GeneralRegister::A => self.a = value,
            GeneralRegister::B => self.b = value,
            GeneralRegister::C => self.c = value,
            GeneralRegister::D => self.d = value,
            GeneralRegister::E => self.e = value,
            GeneralRegister::F => self.f = value,
            GeneralRegister::G => self.g = value,
            GeneralRegister::Aux1 => self.aux1 = value,
            GeneralRegister::Aux2 => self.aux2 = value,
            GeneralRegister::Aux3 => self.aux3 = value,
            GeneralRegister::Carry => self.carry = value,
            GeneralRegister::Low => self.low = value,
            GeneralRegister::High => self.high = value,
            GeneralRegister::ReturnAddress => self.return_address = value,
            GeneralRegister::StackPointer => self.stack_pointer = value,
        }
    }
}