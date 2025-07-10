pub fn shift_left_with_carry(value: u16, shift: u16) -> (u16, u16) {
    /* 
        se 0b_1000_0000_0000_0000 << 2 então:
            - 1º u16 = 0b_0000_0000_0000_0000 (result)
            - 2º u16 = 0b_0000_0000_0000_0010 (carry)
        se 0b_0000_0000_0000_0001 << 32 então:
            - 1º u16 = 0b_0000_0000_0000_0000 (result)
            - 2º u16 = 0b_0000_0000_0000_0000 (carry)
    */
    if shift == 0 {
        return (value, 0);
    }
    if shift >= 16 {
        let carry = (value as u32).checked_shl(shift as u32).unwrap_or(0) >> 16;
        return (0, carry as u16);
    }

    let result = value << shift;
    let carry = value >> (16 - shift);
    (result, carry)
}

pub fn shift_right_with_carry(value: u16, shift: u16) -> (u16, u16) {
    /* 
        se 0b_0000_0000_0000_1111 >> 3 então:
            - 1º u16 = 0b_0000_0000_0000_0001 (result)
            - 2º u16 = 0b_1110_0000_0000_0000 (carry)

        se 0b_0000_0000_0000_0001 >> 32 então:
            - 1º u16 = 0b_0000_0000_0000_0000 (result)
            - 2º u16 = 0b_0000_0000_0000_0000 (carry)
    */
    if shift == 0 {
        return (value, 0);
    }
    if shift >= 16 {
        return (0, 0);
    }

    let result = value >> shift;
    let carry = (value & ((1 << shift) - 1)) << (16 - shift);
    (result, carry)
}