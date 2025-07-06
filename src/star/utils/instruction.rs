#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    Xb, // extend byte -- xb $r1, $r2

    Lab, // load alt byte -- lab $r, $raddress
    Llb, // load low byte -- llb $r, $raddress

    Sab, // store alt byte -- sab $r, $raddress
    Slb, // store low byte -- slb $r, $raddress

    Mulhl, // multiply high low -- mulhl $r1, $r2
    Divhl, // divide high low -- divhl $r1, $r2

    Muluhl, // multiply unsigned high low -- muluhl $r1, $r2
    Divuhl, // divide unsigned high low -- divuhl $r1, $r2

    Not, // not -- not $rd, $rs
    
    Jar, // jump absolute relative -- jar $ra, $a
    // ==== 1111_1111_OOOO_XXXX ====
        // DELETED: Br, // branch relative -- br $r
    // ==== 1111_1111_1111_OOOO ====
        // DELETED: Ret, // return -- ret
    Mcall, // machine call (syscall) -- mcall
        // DELETED: Nope, // (1111_1111_1111_1111) -- nope
}