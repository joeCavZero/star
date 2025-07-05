#[derive(Debug, Clone, PartialEq)]
pub enum PseudoInstruction {
    // ==== Memory Pseudo Instructions ====
    Move, // move -- move $rd, $rs
    Swap, // swap -- swap $r1, $r2
    La, // load address -- la $rd, address
    Lra, // load relative address -- lra $rd, address
    Lxi, // Load extended immediate -- lxi $rd, imm<8>

    Lb, // load byte -- lb $rd, $rs[imm]
    Lw, // load word -- lw $rd, $rs[imm]

    Lbi, // load byte immediate -- lbi $rd, imm
    Lwi, // load word immediate -- lwi $rd, imm

    Sb, // store byte -- sb $rs, $rd[imm]
    Sw, // store word -- sw $rs, $rd[imm]

    Sbi, // store byte immediate -- sbi $rs, imm
    Swi, // store word immediate -- swi $rs, imm

    // ==== Arithmetic Pseudo Instructions ====
    Addi, // add immediate -- addi $rd, $rs, imm
    Subi, // subtract immediate
    Andi, // and immediate
    Ori, // or immediate
    Xori, // xor immediate
    Shli, // shift left immediate
    Shri, // shift right immediate

    Neg, // negate -- neg $rd

    Inc, // increment -- inc $r
    Dec, // decrement -- dec $r

    Mul, // multiply -- mul $rd, $rs, $rt
    Div, // divide -- div $rd, $rs, $rt
    Mod, // modulo -- mod $rd, $rs, $rt

    Muli, // multiply immediate -- muli $rd, $rs, imm
    Divi, // divide immediate -- divi $rd, $rs, imm
    Modi, // modulo immediate -- modi $rd, $rs, imm

    Mului, // multiply unsigned immediate -- mului $rd, $rs, imm
    Divui, // divide unsigned immediate -- divui $rd, $rs, imm
    Modui, // modulo unsigned immediate -- modui $rd, $rs, imm

    // ==== Locality Pseudo Instructions ====
    Pushb, // push byte into memory -- pushb $r
    Pushw, // push word into memory -- pushw $r

    Popb, // pop byte from memory -- popb $r
    Popw, // pop word from memory -- popw $r

    Rpushb, // reverse push byte into memory -- rpushb $r
    Rpushw, // reverse push word into memory -- rpushw $r

    Rpopb, // reverse pop byte from memory -- rpopb $r
    Rpopw, // reverse pop word from memory -- rpopw $r

    Insp, // increment stack pointer -- insp
    Desp, // decrement stack pointer -- dsp

    // ==== Control Flow Pseudo Instructions ====
    Beqa, // branch equal address -- beqa $rs, $rt, address
    Bneqa, // branch not equal address -- bneqa $rs, $rt, address
    Bgta, // branch greater than address -- bgta $rs, $rt, address
    Blta, // branch less than address -- blta $rs, $rt, address

    Bgtua, // branch greater than unsigned address -- bgtua $rs, $rt, address
    Bltua, // branch less than unsigned address -- bltua $rs, $rt, address

    Ba, // branch address -- ba address
}

impl PseudoInstruction {
    pub fn is_addressed(&self) -> bool {
        match self {
            PseudoInstruction::La
            | PseudoInstruction::Beqa
            | PseudoInstruction::Bneqa
            | PseudoInstruction::Bgta
            | PseudoInstruction::Blta
            | PseudoInstruction::Bgtua
            | PseudoInstruction::Bltua => true,
            _ => false,
        }
    }
}