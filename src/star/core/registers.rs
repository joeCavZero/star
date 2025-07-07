use crate::star::utils::Register;

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
    pub high: u16,
    pub low: u16,
    pub return_address: u16,
    pub stack_pointer: u16,

    // ==== OCULT REGISTERS ====
    pub program_counter: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            zero: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            aux1: 0,
            aux2: 0,
            aux3: 0,
            carry: 0,
            high: 0,
            low: 0,
            return_address: 0,
            stack_pointer: 0,
            program_counter: 0,
        }
    }

    pub fn get(&self, register: Register) -> u16 {
        match register {
            Register::Zero => self.zero,
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::F => self.f,
            Register::G => self.g,
            Register::Aux1 => self.aux1,
            Register::Aux2 => self.aux2,
            Register::Aux3 => self.aux3,
            Register::Carry => self.carry,
            Register::High => self.high,
            Register::Low => self.low,
            Register::ReturnAddress => self.return_address,
            Register::StackPointer => self.stack_pointer,
        }
    }

    pub fn set(&mut self, register: Register, value: u16) {
        match register {
            Register::Zero => {}
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::F => self.f = value,
            Register::G => self.g = value,
            Register::Aux1 => self.aux1 = value,
            Register::Aux2 => self.aux2 = value,
            Register::Aux3 => self.aux3 = value,
            Register::Carry => self.carry = value,
            Register::High => self.high = value,
            Register::Low => self.low = value,
            Register::ReturnAddress => self.return_address = value,
            Register::StackPointer => self.stack_pointer = value,
        }
    }
}