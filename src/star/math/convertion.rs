pub fn u16_from_string(s: String) -> Result<u16, String> {
    match s.parse::<u16>() {
        Ok(value) => Ok(value),
        Err(_) => {
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