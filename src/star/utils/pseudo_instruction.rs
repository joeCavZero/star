#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PseudoInstruction {
    // ==== Memory Pseudo Instructions ====
    Nope, // --> add $zero, $zero, $zero
    Move, // move -- move $rd, $rs
    Swap, // swap -- swap $r1, $r2
    La, // load address -- la $rd, address

    Lb, // load byte -- lb $rd, $rs[imm]
    Lw, // load word -- lw $rd, $rs[imm]

    Li, // load immediate -- lwi $rd, imm

    Sb, // store byte -- sb $rs, $rd[imm]
    Sw, // store word -- sw $rs, $rd[imm]

    // ==== Arithmetic Pseudo Instructions ====
    Addi, // add immediate -- addi $rd, $rs, imm
    Subi, // subtract immediate
    Andi, // and immediate
    Ori, // or immediate
    Xori, // xor immediate
    Shli, // shift left immediate
    Shri, // shift right immediate

    Inc, // increment -- inc $r
    Dec, // decrement -- dec $r

    Mul, // multiply -- mul $rd, $rs, $rt
    Div, // divide -- div $rd, $rs, $rt
    Mod, // modulo -- mod $rd, $rs, $rt

    Muli, // multiply immediate -- muli $rd, $rs, imm
    Divi, // divide immediate -- divi $rd, $rs, imm
    Modi, // modulo immediate -- modi $rd, $rs, imm

    // ==== Control Flow Pseudo Instructions ====
    Beqa, // branch equal address -- beqa $rs, $rt, address
    Bneqa, // branch not equal address -- bneqa $rs, $rt, address
    Bgta, // branch greater than address -- bgta $rs, $rt, address
    Blta, // branch less than address -- blta $rs, $rt, address

    Bgtua, // branch greater than unsigned address -- bgtua $rs, $rt, address
    Bltua, // branch less than unsigned address -- bltua $rs, $rt, address

    Ja, // branch address -- ja address
    Jr, // branch relative -- jr $rs

    Ret, // return -- ret
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