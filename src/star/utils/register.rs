#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneralRegister {
    Zero,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    Aux1,
    Aux2,
    Aux3,
    Carry,
    High,
    Low,
    ReturnAddress,
    StackPointer,
}

impl GeneralRegister {
    pub fn code(&self) -> u16 {
        match self {
            GeneralRegister::Zero =>           0b0000_0000_0000_0000,
            GeneralRegister::A =>              0b0000_0000_0000_0001,
            GeneralRegister::B =>              0b0000_0000_0000_0010,
            GeneralRegister::C =>              0b0000_0000_0000_0011,
            GeneralRegister::D =>              0b0000_0000_0000_0100,
            GeneralRegister::E =>              0b0000_0000_0000_0101,
            GeneralRegister::F =>              0b0000_0000_0000_0110,
            GeneralRegister::G =>              0b0000_0000_0000_0111,
            GeneralRegister::Aux1 =>           0b0000_0000_0000_1000,
            GeneralRegister::Aux2 =>           0b0000_0000_0000_1001,
            GeneralRegister::Aux3 =>           0b0000_0000_0000_1010,
            GeneralRegister::Carry =>          0b0000_0000_0000_1011,
            GeneralRegister::Low =>            0b0000_0000_0000_1100,
            GeneralRegister::High =>           0b0000_0000_0000_1101,
            GeneralRegister::ReturnAddress =>  0b0000_0000_0000_1110,
            GeneralRegister::StackPointer =>   0b0000_0000_0000_1111,
        }
    } 

    pub fn from_code(code: u16) -> Self {
        let c = code & 0b0000_0000_0000_1111; // Ensure only the last 4 bits are considered
        match c {
            0b0000_0000_0000_0000 => GeneralRegister::Zero,
            0b0000_0000_0000_0001 => GeneralRegister::A,
            0b0000_0000_0000_0010 => GeneralRegister::B,
            0b0000_0000_0000_0011 => GeneralRegister::C,
            0b0000_0000_0000_0100 => GeneralRegister::D,
            0b0000_0000_0000_0101 => GeneralRegister::E,
            0b0000_0000_0000_0110 => GeneralRegister::F,
            0b0000_0000_0000_0111 => GeneralRegister::G,
            0b0000_0000_0000_1000 => GeneralRegister::Aux1,
            0b0000_0000_0000_1001 => GeneralRegister::Aux2,
            0b0000_0000_0000_1010 => GeneralRegister::Aux3,
            0b0000_0000_0000_1011 => GeneralRegister::Carry,
            0b0000_0000_0000_1100 => GeneralRegister::Low,
            0b0000_0000_0000_1101 => GeneralRegister::High,
            0b0000_0000_0000_1110 => GeneralRegister::ReturnAddress,
            0b0000_0000_0000_1111 => GeneralRegister::StackPointer,
            _ => GeneralRegister::Zero,
        }
    }
}