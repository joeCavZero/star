pub fn split_u16_to_strings(value: u16) -> (String, String) {
    // split the u16 bits into two u8 values
    unsafe {
        let (low, high) = std::mem::transmute::<u16, (u8, u8)>(value);
        return (
            format!("0b{:08b}", low), format!("0b{:08b}", high),
        )
    }
}