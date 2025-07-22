use crate::star::utils::Format;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    // ==== OOOO_XXXX_YYYY_ZZZZ ====

    Add, // addition -- add $rd, $r1, $r2
    Sub, // subtraction -- sub $rd, $r1, $r2
    And, // and -- and $rd, $r1, $r2
    Or, // or -- or $rd, $r1, $r2
    Xor, // xor -- xor $rd, $r1, $r2
    Shl, // shift left -- shl $rd, $r1, $r2
    Shr, // shift right -- shr $rd, $r1, $r2

    Lai, // load alt immediate -- lai $rd, $imm<8>
    Lli, // load low immediate -- lli $rd, $imm<8>

    Beqr, // branch equal -- beqr $r1, $r2, $rt
    Bneqr, // branch not equal -- bneqr $r1, $r2, $rt
    Bgtr, // branch greater than -- bgtr $r1, $r2, $rt
    Bltr, // branch less than -- bltr $r1, $r2, $rt

    Bgtur, // branch greater than unsigned relative -- bgtur $r1, $r2, $rt
    Bltur, // branch less than unsigned relative -- bltur $r1, $r2, $rt

    // ==== 1111_OOOO_XXXX_YYYY ====

    Mulhl, // multiply high low -- mulhl $r1, $r2
    Divhl, // divide high low -- divhl $r1, $r2

    Muluhl, // multiply unsigned high low -- muluhl $r1, $r2
    Divuhl, // divide unsigned high low -- divuhl $r1, $r2

    Not, // not -- not $rd, $rs

    Xlb, // extend low byte -- xlb $r1, $r2

    Lab, // load alt byte -- lab $r, $raddress
    Llb, // load low byte -- llb $r, $raddress

    Sab, // store alt byte -- sab $r, $raddress
    Slb, // store low byte -- slb $r, $raddress
    
    // ==== 1111_1111_OOOO_XXXX ====
        // DELETED: Br, // branch relative -- br $r
        J, // jump -- j $rs
    // ==== 1111_1111_1111_OOOO ====
        // DELETED: Ret, // return -- ret
    Mcall, // machine call (syscall) -- mcall
        // DELETED: Nope, // (1111_1111_1111_1111) -- nope
}

impl Instruction {
    pub fn format(&self) -> Format {
        match self {
            // ==== TRINITY ====
            Instruction::Add
            | Instruction::Sub
            | Instruction::And
            | Instruction::Or
            | Instruction::Xor
            | Instruction::Shl
            | Instruction::Shr
            
            | Instruction::Beqr
            | Instruction::Bneqr
            | Instruction::Bgtr
            | Instruction::Bltr
            | Instruction::Bgtur
            | Instruction::Bltur
            => Format::Trinity,

            // ==== HIME ====
            Instruction::Lai
            | Instruction::Lli
            => Format::Hime,
            // ==== PAIR ====
            Instruction::Mulhl
            | Instruction::Divhl
            | Instruction::Muluhl
            | Instruction::Divuhl
            | Instruction::Not
            | Instruction::Xlb
            | Instruction::Lab
            | Instruction::Llb
            | Instruction::Sab
            | Instruction::Slb
            => Format::Pair,

            // ==== CLOVER ====
            Instruction::J 
            => Format::Clover,
            // ==== ARK ====
            Instruction::Mcall 
            => Format::Ark,
        }
    }
    pub fn opcode(&self) -> u16 {
        match self {
            // ==== TRINITY ====
            Instruction::Add => 0b0000_0000_0000_0000,
            Instruction::Sub => 0b0000_0000_0000_0001,
            Instruction::And => 0b0000_0000_0000_0010,
            Instruction::Or => 0b0000_0000_0000_0011,
            Instruction::Xor => 0b0000_0000_0000_0100,
            Instruction::Shl => 0b0000_0000_0000_0101,
            Instruction::Shr => 0b0000_0000_0000_0110,

            Instruction::Lai => 0b0000_0000_0000_0111,
            Instruction::Lli => 0b0000_0000_0000_1000,

            Instruction::Beqr => 0b0000_0000_0000_1001,
            Instruction::Bneqr => 0b0000_0000_0000_1010,
            Instruction::Bgtr => 0b0000_0000_0000_1011,
            Instruction::Bltr => 0b0000_0000_0000_1100,
            Instruction::Bgtur => 0b0000_0000_0000_1101,
            Instruction::Bltur => 0b0000_0000_0000_1110,

            // ==== PAIR ====
            Instruction::Mulhl => 0b0000_0000_0000_1111,
            Instruction::Divhl => 0b0000_0000_0001_1111,
            Instruction::Muluhl => 0b0000_0000_0010_1111,
            Instruction::Divuhl => 0b0000_0000_0011_1111,

            Instruction::Not => 0b0000_0000_0100_1111,

            Instruction::Xlb => 0b0000_0000_0101_1111,
            Instruction::Lab => 0b0000_0000_0110_1111,
            Instruction::Llb => 0b0000_0000_0111_1111,
            Instruction::Sab => 0b0000_0000_1000_1111,
            Instruction::Slb => 0b0000_0000_1001_1111,

            // ==== CLOVER ====
            Instruction::J => 0b0000_0000_1111_1111,

            // ==== ARK ====
            Instruction::Mcall => 0b0000_1111_1111_1111,

        }
    }

    pub fn from_opcode(opcode: u16) -> Option<Self> {
        match opcode {
            0b_0000_0000_0000_0000 => Some(Instruction::Add),
            0b_0000_0000_0000_0001 => Some(Instruction::Sub),
            0b_0000_0000_0000_0010 => Some(Instruction::And),
            0b_0000_0000_0000_0011 => Some(Instruction::Or),
            0b_0000_0000_0000_0100 => Some(Instruction::Xor),
            0b_0000_0000_0000_0101 => Some(Instruction::Shl),
            0b_0000_0000_0000_0110 => Some(Instruction::Shr),

            0b_0000_0000_0000_0111 => Some(Instruction::Lai),
            0b_0000_0000_0000_1000 => Some(Instruction::Lli),

            0b_0000_0000_0000_1001 => Some(Instruction::Beqr),
            0b_0000_0000_0000_1010 => Some(Instruction::Bneqr),
            0b_0000_0000_0000_1011 => Some(Instruction::Bgtr),
            0b_0000_0000_0000_1100 => Some(Instruction::Bltr),
            0b_0000_0000_0000_1101 => Some(Instruction::Bgtur),
            0b_0000_0000_0000_1110 => Some(Instruction::Bltur),
            
            // ==== PAIR ====Some(
            0b_0000_0000_0000_1111 => Some(Instruction::Mulhl),
            0b_0000_0000_0001_1111 => Some(Instruction::Divhl),
            0b_0000_0000_0010_1111 => Some(Instruction::Muluhl),
            0b_0000_0000_0011_1111 => Some(Instruction::Divuhl),

            0b_0000_0000_0100_1111 => Some(Instruction::Not),

            0b_0000_0000_0101_1111 => Some(Instruction::Xlb),
            0b_0000_0000_0110_1111 => Some(Instruction::Lab),
            0b_0000_0000_0111_1111 => Some(Instruction::Llb),
            0b_0000_0000_1000_1111 => Some(Instruction::Sab),
            0b_0000_0000_1001_1111 => Some(Instruction::Slb),

            // ==== CLOVER ====
            0b_0000_0000_1111_1111 => Some(Instruction::J),
            
            // ==== ARK ====
            0b_0000_1111_1111_1111 => Some(Instruction::Mcall),

            _ => None,
        }
    }
}