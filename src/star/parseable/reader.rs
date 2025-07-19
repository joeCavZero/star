use crate::star::{parseable::Sequence, utils::*};

pub fn read_r_r_sequence(ptokens: &Vec<PositionedToken>, start_index: usize, base_position: Position) -> Result<Sequence, (String, Position)> {
    match ptokens.get(start_index) {
        Some(tk1) => {
            if let Token::GeneralRegister(_) = tk1.token {
                match ptokens.get(start_index + 1) {
                    Some(comma) => {
                        if let Token::Comma = comma.token {
                            match ptokens.get(start_index + 2) {
                                Some(tk2) => {
                                    if let Token::GeneralRegister(_) = tk2.token {
                                        Ok(Sequence::Two(tk1.clone(), tk2.clone()))
                                    } else {
                                        Err(("Expected a register after comma in this sequence".to_string(), tk2.position))
                                    }
                                }
                                None => Err(("Expected a register after comma in this sequence".to_string(), comma.position)),
                            }
                        } else {
                            Err(("Expected a comma after register in this sequence".to_string(), comma.position))
                        }
                    }
                    None => Err(("Expected a comma after register in this sequence".to_string(), tk1.position)),
                }
            } else {
                Err(("Expected a register at the beginning of this sequence".to_string(), base_position))
            }
        }
        None => Err(("Expected a register at the beginning of this sequence".to_string(), base_position)),
    }
}

pub fn read_r_n_sequence(ptokens: &Vec<PositionedToken>, start_index: usize, base_position: Position) -> Result<Sequence, (String, Position)> {
    match ptokens.get(start_index) {
        Some(tk1) => {
            if let Token::GeneralRegister(_) = tk1.token {
                match ptokens.get(start_index + 1) {
                    Some(comma) => {
                        if let Token::Comma = comma.token {
                            match ptokens.get(start_index + 2) {
                                Some(tk2) => {
                                    if let Token::NumberLiteral(_) = tk2.token {
                                        Ok(Sequence::Two(tk1.clone(), tk2.clone()))
                                    } else {
                                        Err(("Expected a number after comma in this sequence".to_string(), tk2.position))
                                    }
                                }
                                None => Err(("Expected a number after comma in this sequence".to_string(), comma.position)),
                            }
                        } else {
                            Err(("Expected a comma after register in this sequence".to_string(), comma.position))
                        }
                    }
                    None => Err(("Expected a comma after register in this sequence".to_string(), tk1.position)),
                }
            } else {
                Err(("Expected a register at the beginning of this sequence".to_string(), base_position))
            }
        }
        None => Err(("Expected a register at the beginning of this sequence".to_string(), base_position)),
    }
}

pub fn read_r_r_r_sequence(ptokens: &Vec<PositionedToken>, start_index: usize, base_position: Position) -> Result<Sequence, (String, Position)> {
    match ptokens.get(start_index) {
        Some(tk1) => {
            if let Token::GeneralRegister(_) = tk1.token {
                match ptokens.get(start_index + 1) {
                    Some(comma1) => {
                        if let Token::Comma = comma1.token {
                            match ptokens.get(start_index + 2) {
                                Some(tk2) => {
                                    if let Token::GeneralRegister(_) = tk2.token {
                                        match ptokens.get(start_index + 3) {
                                            Some(comma2) => {
                                                if let Token::Comma = comma2.token {
                                                    match ptokens.get(start_index + 4) {
                                                        Some(tk3) => {
                                                            if let Token::GeneralRegister(_) = tk3.token {
                                                                Ok( Sequence::Three(tk1.clone(), tk2.clone(), tk3.clone() ))
                                                            } else {
                                                                Err(("Expected a register after second comma in this sequence".to_string(), tk3.position))
                                                            }
                                                        }
                                                        None => Err(("Expected a register after second comma in this sequence".to_string(), comma2.position)),
                                                    }
                                                } else {
                                                    Err(("Expected a comma after second register in this sequence".to_string(), comma2.position))
                                                }
                                            }
                                            None => Err(("Expected a comma after first register in this sequence".to_string(), tk2.position)),
                                        }
                                    } else {
                                        Err(("Expected a register after first comma in this sequence".to_string(), tk2.position))
                                    }
                                }
                                None => Err(("Expected a register after first comma in this sequence".to_string(), comma1.position)),
                            }
                        } else {
                            Err(("Expected a comma after first register in this sequence".to_string(), comma1.position))
                        }
                    }
                    None => Err(("Expected a comma after first register in this sequence".to_string(), tk1.position)),
                }
            } else {
                Err(("Expected a register at the beginning of this sequence".to_string(), base_position))
            }
        }
        None => Err(("Expected a register at the beginning of this sequence".to_string(), base_position)),
    }
}

pub fn read_r_id_sequence(ptokens: &Vec<PositionedToken>, start_index: usize, base_position: Position) -> Result<Sequence, (String, Position)> {
    match ptokens.get(start_index) {
        Some(tk1) => {
            if let Token::GeneralRegister(_) = tk1.token {
                match ptokens.get(start_index + 1) {
                    Some(comma) => {
                        if let Token::Comma = comma.token {
                            match ptokens.get(start_index + 2) {
                                Some(tk2) => {
                                    if let Token::Identifier(_) = tk2.token {
                                        Ok(Sequence::Two(tk1.clone(), tk2.clone()))
                                    } else {
                                        Err(("Expected an identifier after comma in this sequence".to_string(), tk2.position))
                                    }
                                }
                                None => Err(("Expected an identifier after comma in this sequence".to_string(), comma.position)),
                            }
                        } else {
                            Err(("Expected a comma after register in this sequence".to_string(), comma.position))
                        }
                    }
                    None => Err(("Expected a comma after register in this sequence".to_string(), tk1.position)),
                }
            } else {
                Err(("Expected a register at the beginning of this sequence".to_string(), base_position))
            }
        }
        None => Err(("Expected a register at the beginning of this sequence".to_string(), base_position)),
    }
}

pub fn read_r_r_br_n_br(ptokens: &Vec<PositionedToken>, start_index: usize, base_position: Position) -> Result<Sequence, (String, Position)> {
    // read reg reg square_bracket number square_bracket
    // e.g.: lw $r1, $r2[10]
    match ptokens.get(start_index) {
        Some(tk1) => {
            if let Token::GeneralRegister(_) = tk1.token {
                match ptokens.get(start_index + 1) {
                    Some(comma) => {
                        if let Token::Comma = comma.token {
                            match ptokens.get(start_index + 2) {
                                Some(tk2) => {
                                    if let Token::GeneralRegister(_) = tk2.token {
                                        match ptokens.get(start_index + 3) {
                                            Some(left_square_bracket) => {
                                                if let Token::LeftSquareBracket = left_square_bracket.token {
                                                    match ptokens.get(start_index + 4) {
                                                        Some(tk3) => {
                                                            if let Token::NumberLiteral(_) = tk3.token {
                                                                match ptokens.get(start_index + 5) {
                                                                    Some(right_square_bracket) => {
                                                                        if let Token::RightSquareBracket = right_square_bracket.token {
                                                                            Ok(Sequence::Three(tk1.clone(), tk2.clone(), tk3.clone()))
                                                                        } else {
                                                                            Err(("Expected a right square bracket after number in this sequence".to_string(), right_square_bracket.position))
                                                                        }
                                                                    }
                                                                    None => Err(("Expected a right square bracket after number in this sequence".to_string(), tk3.position)),
                                                                }
                                                            } else {
                                                                Err(("Expected a number after left square bracket in this sequence".to_string(), tk3.position))
                                                            }
                                                        }
                                                        None => Err(("Expected a number after left square bracket in this sequence".to_string(), left_square_bracket.position)),
                                                    }
                                                } else {
                                                    Err(("Expected a left square bracket after second register in this sequence".to_string(), left_square_bracket.position))
                                                }
                                            }
                                            None => Err(("Expected a left square bracket after second register in this sequence".to_string(), tk2.position)),
                                        }
                                    } else {
                                        Err(("Expected a register after comma in this sequence".to_string(), tk2.position))
                                    }
                                }
                                None => Err(("Expected a register after comma in this sequence".to_string(), comma.position)),
                            }
                        } else {
                            Err(("Expected a comma after first register in this sequence".to_string(), comma.position))
                        }
                    }
                    None => Err(("Expected a comma after first register in this sequence".to_string(), tk1.position)),
                }
            } else {
                Err(("Expected a register at the beginning of this sequence".to_string(), base_position))
            }
        }
        None => Err(("Expected a register at the beginning of this sequence".to_string(), base_position)),
    }
}


pub fn read_r_id_br_n_br(ptokens: &Vec<PositionedToken>, start_index: usize, base_position: Position) -> Result<Sequence, (String, Position)> {
    // read reg comma identifier square_bracket number square_bracket
    // e.g.: $r1, label[10]
    match ptokens.get(start_index) {
        Some(tk1) => {
            if let Token::GeneralRegister(_) = tk1.token {
                match ptokens.get(start_index + 1) {
                    Some(comma) => {
                        if let Token::Comma = comma.token {
                            match ptokens.get(start_index + 2) {
                                Some(tk2) => {
                                    if let Token::Identifier(_) = tk2.token {
                                        match ptokens.get(start_index + 3) {
                                            Some(left_square_bracket) => {
                                                if let Token::LeftSquareBracket = left_square_bracket.token {
                                                    match ptokens.get(start_index + 4) {
                                                        Some(tk3) => {
                                                            if let Token::NumberLiteral(_) = tk3.token {
                                                                match ptokens.get(start_index + 5) {
                                                                    Some(right_square_bracket) => {
                                                                        if let Token::RightSquareBracket = right_square_bracket.token {
                                                                            Ok(Sequence::Three(tk1.clone(), tk2.clone(), tk3.clone()))
                                                                        } else {
                                                                            Err(("Expected a right square bracket after number in this sequence".to_string(), right_square_bracket.position))
                                                                        }
                                                                    }
                                                                    None => Err(("Expected a right square bracket after number in this sequence".to_string(), tk3.position)),
                                                                }
                                                            } else {
                                                                Err(("Expected a number inside brackets in this sequence".to_string(), tk3.position))
                                                            }
                                                        }
                                                        None => Err(("Expected a number inside brackets in this sequence".to_string(), left_square_bracket.position)),
                                                    }
                                                } else {
                                                    Err(("Expected a left square bracket after identifier in this sequence".to_string(), left_square_bracket.position))
                                                }
                                            }
                                            None => Err(("Expected a left square bracket after identifier in this sequence".to_string(), tk2.position)),
                                        }
                                    } else {
                                        Err(("Expected an identifier after comma in this sequence".to_string(), tk2.position))
                                    }
                                }
                                None => Err(("Expected an identifier after comma in this sequence".to_string(), comma.position)),
                            }
                        } else {
                            Err(("Expected a comma after first register in this sequence".to_string(), comma.position))
                        }
                    }
                    None => Err(("Expected a comma after first register in this sequence".to_string(), tk1.position)),
                }
            } else {
                Err(("Expected a register at the beginning of this sequence".to_string(), base_position))
            }
        }
        None => Err(("Expected a register at the beginning of this sequence".to_string(), base_position)),
    }
}

pub fn read_r_sequence(ptokens: &Vec<PositionedToken>, start_index: usize, base_position: Position) -> Result<Sequence, (String, Position)> {
    match ptokens.get(start_index) {
        Some(tk1) => {
            if let Token::GeneralRegister(_) = tk1.token {
                Ok(Sequence::One(tk1.clone()))
            } else {
                Err(("Expected a register in this sequence".to_string(), base_position))
            }
        }
        None => Err(("Expected a register in this sequence".to_string(), base_position)),
    }
}

pub fn read_id_sequence(ptokens: &Vec<PositionedToken>, start_index: usize, base_position: Position) -> Result<Sequence, (String, Position)> {
    match ptokens.get(start_index) {
        Some(tk1) => {
            if let Token::Identifier(_) = tk1.token {
                Ok(Sequence::One(tk1.clone()))
            } else {
                Err(("Expected an identifier in this sequence".to_string(), base_position))
            }
        }
        None => Err(("Expected an identifier in this sequence".to_string(), base_position)),
    }
}

pub fn read_r_r_id_sequence(ptokens: &Vec<PositionedToken>, start_index: usize, base_position: Position) -> Result<Sequence, (String, Position)> {
    match ptokens.get(start_index) {
        Some(tk1) => {
            if let Token::GeneralRegister(_) = tk1.token {
                match ptokens.get(start_index + 1) {
                    Some(comma1) => {
                        if let Token::Comma = comma1.token {
                            match ptokens.get(start_index + 2) {
                                Some(tk2) => {
                                    if let Token::GeneralRegister(_) = tk2.token {
                                        match ptokens.get(start_index + 3) {
                                            Some(comma2) => {
                                                if let Token::Comma = comma2.token {
                                                    match ptokens.get(start_index + 4) {
                                                        Some(tk3) => {
                                                            if let Token::Identifier(_) = tk3.token {
                                                                Ok(Sequence::Three(tk1.clone(), tk2.clone(), tk3.clone()))
                                                            } else {
                                                                Err(("Expected an identifier after second comma in this sequence".to_string(), tk3.position))
                                                            }
                                                        }
                                                        None => Err(("Expected an identifier after second comma in this sequence".to_string(), comma2.position)),
                                                    }
                                                } else {
                                                    Err(("Expected a comma after second register in this sequence".to_string(), comma2.position))
                                                }
                                            }
                                            None => Err(("Expected a comma after first register in this sequence".to_string(), tk2.position)),
                                        }
                                    } else {
                                        return Err(("Expected a register after comma in this sequence".to_string(), tk2.position));
                                    }
                                }
                                None => Err(("Expected an register after comma in this sequence".to_string(), comma1.position)),
                            }
                        } else {
                            Err(("Expected a comma after register in this sequence".to_string(), comma1.position))
                        }
                    }
                    None => Err(("Expected a comma after register in this sequence".to_string(), tk1.position)),
                }
            } else {
                Err(("Expected a register at the beginning of this sequence".to_string(), base_position))
            }
        }
        None => Err(("Expected a register at the beginning of this sequence".to_string(), base_position)),
    }
}

pub fn read_r_r_n_sequence(ptokens: &Vec<PositionedToken>, start_index: usize, base_position: Position) -> Result<Sequence, (String, Position)> {
    match ptokens.get(start_index) {
        Some(tk1) => {
            if let Token::GeneralRegister(_) = tk1.token {
                match ptokens.get(start_index + 1) {
                    Some(comma1) => {
                        if let Token::Comma = comma1.token {
                            match ptokens.get(start_index + 2) {
                                Some(tk2) => {
                                    if let Token::GeneralRegister(_) = tk2.token {
                                        match ptokens.get(start_index + 3) {
                                            Some(comma2) => {
                                                if let Token::Comma = comma2.token {
                                                    match ptokens.get(start_index + 4) {
                                                        Some(tk3) => {
                                                            if let Token::NumberLiteral(_) = tk3.token {
                                                                Ok(Sequence::Three(tk1.clone(), tk2.clone(), tk3.clone()))
                                                            } else {
                                                                Err(("Expected a number after second comma in this sequence".to_string(), tk3.position))
                                                            }
                                                        }
                                                        None => Err(("Expected a number after second comma in this sequence".to_string(), comma2.position)),
                                                    }
                                                } else {
                                                    Err(("Expected a comma after second register in this sequence".to_string(), comma2.position))
                                                }
                                            }
                                            None => Err(("Expected a comma after second register in this sequence".to_string(), tk2.position)),
                                        }
                                    } else {
                                        Err(("Expected a register after comma in this sequence".to_string(), tk2.position))
                                    }
                                }
                                None => Err(("Expected a register after comma in this sequence".to_string(), comma1.position)),
                            }
                        } else {
                            Err(("Expected a comma after first register in this sequence".to_string(), comma1.position))
                        }
                    }
                    None => Err(("Expected a comma after first register in this sequence".to_string(), tk1.position)),
                }
            } else {
                Err(("Expected a register at the beginning of this sequence".to_string(), base_position))
            }
        }
        None => Err(("Expected a register at the beginning of this sequence".to_string(), base_position)),
    }
}