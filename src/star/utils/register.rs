#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
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

impl Register {
    pub fn code(&self) -> u16 {
        match self {
            Register::Zero =>           0b0000_0000_0000_0000,
            Register::A =>              0b0000_0000_0000_0001,
            Register::B =>              0b0000_0000_0000_0010,
            Register::C =>              0b0000_0000_0000_0011,
            Register::D =>              0b0000_0000_0000_0100,
            Register::E =>              0b0000_0000_0000_0101,
            Register::F =>              0b0000_0000_0000_0110,
            Register::G =>              0b0000_0000_0000_0111,
            Register::Aux1 =>           0b0000_0000_0000_1000,
            Register::Aux2 =>           0b0000_0000_0000_1001,
            Register::Aux3 =>           0b0000_0000_0000_1010,
            Register::Carry =>          0b0000_0000_0000_1011,
            Register::High =>           0b0000_0000_0000_1100,
            Register::Low =>            0b0000_0000_0000_1101,
            Register::ReturnAddress =>  0b0000_0000_0000_1110,
            Register::StackPointer =>   0b0000_0000_0000_1111,
        }
    } 

    pub fn from_code(code: u16) -> Self {
        match code {
            0b0000_0000_0000_0000 => Register::Zero,
            0b0000_0000_0000_0001 => Register::A,
            0b0000_0000_0000_0010 => Register::B,
            0b0000_0000_0000_0011 => Register::C,
            0b0000_0000_0000_0100 => Register::D,
            0b0000_0000_0000_0101 => Register::E,
            0b0000_0000_0000_0110 => Register::F,
            0b0000_0000_0000_0111 => Register::G,
            0b0000_0000_0000_1000 => Register::Aux1,
            0b0000_0000_0000_1001 => Register::Aux2,
            0b0000_0000_0000_1010 => Register::Aux3,
            0b0000_0000_0000_1011 => Register::Carry,
            0b0000_0000_0000_1100 => Register::High,
            0b0000_0000_0000_1101 => Register::Low,
            0b0000_0000_0000_1110 => Register::ReturnAddress,
            0b0000_0000_0000_1111 => Register::StackPointer,
            _ => unreachable!(),
            
        }
    }
}