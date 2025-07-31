use std::mem::transmute;

pub fn u8_from_string(string: String) -> Result<u8, String> {
    let s = string.replace("_", "");
    match s.parse::<u8>() {
        Ok(value) => Ok(value),
        Err(_) => {
            match s.parse::<i8>() {
                Ok(value) => {
                    return Ok(i8::cast_unsigned(value));
                }
                Err(_) => {}
            }
            if s.to_lowercase().starts_with("0x") && s.len() > 2 {
                let lowercased = s.to_lowercase();
                let hex_digits = &lowercased[2..];
                let mut res: u8 = 0;
                for c in hex_digits.chars() {
                    let digit = match c {
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        '3' => 3,
                        '4' => 4,
                        '5' => 5,
                        '6' => 6,
                        '7' => 7,
                        '8' => 8,
                        '9' => 9,
                        'a' => 10,
                        'b' => 11,
                        'c' => 12,
                        'd' => 13,
                        'e' => 14,
                        'f' => 15,
                        _ => return Err(format!("Invalid hex digit: {}", c)),
                    };

                    match res.checked_mul(16) {
                        Some(val) => {
                            match val.checked_add(digit) {
                                Some(v) => res = v,
                                None => return Err("Overflow in hex conversion".to_string()),
                            }
                        }
                        None => return Err("Overflow in hex conversion".to_string()),
                    }
                }
                Ok(res)
            } else if s.to_lowercase().starts_with("0b") && s.len() > 2 {
                let binary_digits = &s[2..];
                let mut res: u8 = 0;
                for c in binary_digits.chars() {
                    let digit = match c {
                        '0' => 0,
                        '1' => 1,
                        _ => return Err(format!("Invalid binary digit: {}", c)),
                    };

                    match res.checked_mul(2) {
                        Some(val) => {
                            match val.checked_add(digit) {
                                Some(v) => res = v,
                                None => return Err("Overflow in binary conversion".to_string()),
                            }
                        }
                        None => return Err("Overflow in binary conversion".to_string()),
                    }
                }
                Ok(res)
            } else {
                Err(format!("Invalid number format: {}", s))
            }
        }
    }
}

pub fn u16_from_string(string: String) -> Result<u16, String> {
    let s = string.replace("_", "");
    match s.parse::<u16>() {
        Ok(value) => Ok(value),
        Err(_) => {
            match s.parse::<i16>() {
                Ok(value) => {
                    return Ok(i16::cast_unsigned(value));
                }
                Err(_) => {}
            }
            if s.to_lowercase().starts_with("0x") && s.len() > 2 {
                let lowercased = s.to_lowercase();
                let hex_digits = &lowercased[2..];
                let mut res: u16 = 0;
                for c in hex_digits.chars() {
                    let digit = match c {
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        '3' => 3,
                        '4' => 4,
                        '5' => 5,
                        '6' => 6,
                        '7' => 7,
                        '8' => 8,
                        '9' => 9,
                        'a' => 10,
                        'b' => 11,
                        'c' => 12,
                        'd' => 13,
                        'e' => 14,
                        'f' => 15,
                        _ => return Err(format!("Invalid hex digit: {}", c)),
                    };

                    match res.checked_mul(16) {
                        Some(val) => {
                            match val.checked_add(digit) {
                                Some(v) => res = v,
                                None => return Err("Overflow in hex conversion".to_string()),
                            }
                        }
                        None => return Err("Overflow in hex conversion".to_string()),
                    }
                }
                Ok(res)
            } else if s.to_lowercase().starts_with("0b") && s.len() > 2 {
                let binary_digits = &s[2..];
                let mut res: u16 = 0;
                for c in binary_digits.chars() {
                    let digit = match c {
                        '0' => 0,
                        '1' => 1,
                        _ => return Err(format!("Invalid binary digit: {}", c)),
                    };

                    match res.checked_mul(2) {
                        Some(val) => {
                            match val.checked_add(digit) {
                                Some(v) => res = v,
                                None => return Err("Overflow in binary conversion".to_string()),
                            }
                        }
                        None => return Err("Overflow in binary conversion".to_string()),
                    }
                }
                Ok(res)
            } else {
                Err(format!("Invalid number format: {}", s))
            }
        }
    }
}

pub fn u32_from_string(string: String) -> Result<u32, String> {
    let s = string.replace("_", "");
    match s.parse::<u32>() {
        Ok(value) => Ok(value),
        Err(_) => {
            match s.parse::<i32>() {
                Ok(value) => {
                    return Ok(i32::cast_unsigned(value));
                }
                Err(_) => {}
            }
            if s.to_lowercase().starts_with("0x") && s.len() > 2 {
                let lowercased = s.to_lowercase();
                let hex_digits = &lowercased[2..];
                let mut res: u32 = 0;
                for c in hex_digits.chars() {
                    let digit = match c {
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        '3' => 3,
                        '4' => 4,
                        '5' => 5,
                        '6' => 6,
                        '7' => 7,
                        '8' => 8,
                        '9' => 9,
                        'a' => 10,
                        'b' => 11,
                        'c' => 12,
                        'd' => 13,
                        'e' => 14,
                        'f' => 15,
                        _ => return Err(format!("Invalid hex digit: {}", c)),
                    };

                    match res.checked_mul(16) {
                        Some(val) => {
                            match val.checked_add(digit) {
                                Some(v) => res = v,
                                None => return Err("Overflow in hex conversion".to_string()),
                            }
                        }
                        None => return Err("Overflow in hex conversion".to_string()),
                    }
                }
                Ok(res)
            } else if s.to_lowercase().starts_with("0b") && s.len() > 2 {
                let binary_digits = &s[2..];
                let mut res: u32 = 0;
                for c in binary_digits.chars() {
                    let digit = match c {
                        '0' => 0,
                        '1' => 1,
                        _ => return Err(format!("Invalid binary digit: {}", c)),
                    };

                    match res.checked_mul(2) {
                        Some(val) => {
                            match val.checked_add(digit) {
                                Some(v) => res = v,
                                None => return Err("Overflow in binary conversion".to_string()),
                            }
                        }
                        None => return Err("Overflow in binary conversion".to_string()),
                    }
                }
                Ok(res)
            } else {
                Err(format!("Invalid number format: {}", s))
            }
        }
    }
}

pub fn extend_sign_from_u16_to_u32(value: u16) -> u32 {
    if value & 0b_1000_0000_0000_0000 != 0 {
        unsafe {
            transmute::<(u16, u16), u32>((value, 0xFFFF))
        }
    } else {
        unsafe {
            transmute::<(u16, u16), u32>((value, 0x0000))
        }
    }
}

pub fn extend_zero_from_u16_to_u32(value: u16) -> u32 {
    unsafe {
        transmute::<(u16, u16), u32>((value, 0x0000))
    }
}