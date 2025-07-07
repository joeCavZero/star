pub fn shift_left_with_carry(value: u16, shift: u16) -> (u16, u16) {
    if shift == 0 {
        return (value, 0);
    }
    let shift = shift % 32; // Normalize shift to avoid undefined behavior for large shifts
    let res = if shift >= 16 { 0 } else { value.wrapping_shl(shift as u32) };
    let carry = if shift == 16 {
        value // For shift = 16, carry is the original value
    } else if shift > 16 {
        if shift % 16 == 0 {
            0 // For shifts like 32, 48, etc., carry is 0
        } else {
            value.wrapping_shl((shift % 16) as u32) & ((1 << (shift % 16)) - 1)
        }
    } else {
        (value >> (16 - shift)) & ((1 << shift) - 1)
    };
    (res, carry)
}

pub fn shift_right_with_carry(value: u16, shift: u16) -> (u16, u16) {
    if shift == 0 {
        return (value, 0);
    }
    let shift = shift % 32; // Normalize shift
    let res = if shift >= 16 { 0 } else { value.wrapping_shr(shift as u32) };
    let carry = if shift >= 16 {
        value // For shift >= 16, carry is the original value
    } else {
        value & ((1 << shift) - 1)
    };
    (res, carry)
}