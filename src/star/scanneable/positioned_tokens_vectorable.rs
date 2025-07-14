use crate::star::utils::*;

pub trait PositionedTokensVectorable {
    fn push_positioned_token(
        &mut self,
        token_string: String,
        file_id: u32,
        line: u32,
        column: Option<u32>,
    ) -> Result<(), String>;

    fn scan_macro_definition_head(
        &self,
        vector_offset: usize,
        identifier_position: Position,
    ) -> Result<(Vec<PositionedToken>, usize), (String, Position)>;

    fn scan_macro_sequence(&self, start_index: usize, identifier_line: u32) -> Result<(Vec<PositionedToken>, usize), (String, Position)>;

    fn scan_macro_calling_head(
        &self,
        vector_offset: usize,
        identifier_position: Position,
    ) -> Result<(Vec<PositionedToken>, usize), (String, Position)>;
}

impl PositionedTokensVectorable for Vec<PositionedToken> {
    fn push_positioned_token(
        &mut self,
        token_string: String,
        file_id: u32,
        line: u32,
        column: Option<u32>,
    ) -> Result<(), String> {
        let tkn = Token::from_string(token_string);
        match tkn {
            Ok(token) => {
                self.push(PositionedToken {
                    token,
                    position: Position::new(
                        file_id,
                        line,
                        column,
                    )
                });
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn scan_macro_definition_head(
        &self,
        vector_offset: usize,
        identifier_position: Position,
    ) -> Result<(Vec<PositionedToken>, usize), (String, Position)> {
        // scan "(%arg1, %arg2, ...)" (this is a macro definition head)
        //      LeftParenthesis, Comma, MacroArgIdentifier, ... RightParenthesis
        // and returns a vector of %arg1, %arg2, ... and the tokens positions to jump
        /*
            possible cases: (%a and %b are macro arguments definitions type)
                ( %a, %b ) -> ([%a, %b], 5) // 5 is the number of tokens readen
                () -> ([], 2)
                ( %a, %b, ) -> error
                ( %a, %b, %c ) -> ([%a, %b, %c], 7)
                ( %a, %b, , ) -> error
                ( %a, , ) -> error
                ( , ) -> error
                
        
         */
        let mut head: Vec<PositionedToken> = Vec::new();
        let mut ptkns_found_quantity = 0;

        let mut is_parenthesis_head = false;
        let mut ptoken_counter = vector_offset;
        let mut last_position = identifier_position.clone();
        let mut has_comma = false;
        
        while ptoken_counter < self.len() {
            let ptkn = match self.get(ptoken_counter) {
                Some(ptkn) => ptkn,
                None => break,
            };

            match ptkn.token.clone() {
                Token::LeftParenthesis => {
                    if !is_parenthesis_head {
                        is_parenthesis_head = true;
                        ptkns_found_quantity += 1;
                    } else {
                        return Err((
                            "Unexpected left parenthesis in macro definition head".to_string(),
                            ptkn.position.clone(),
                        ));
                    }
                }

                Token::RightParenthesis => {
                    if !is_parenthesis_head {
                        return Err((
                            "Unexpected right parenthesis in macro definition head".to_string(),
                            ptkn.position.clone(),
                        ));
                    }
                    if has_comma {
                        return Err((
                            "Unexpected right parenthesis in macro definition head, expected macro argument".to_string(),
                            ptkn.position.clone(),
                        ));
                    } else {
                        ptkns_found_quantity += 1;
                        return Ok((head, ptkns_found_quantity));
                    }
                }
                Token::Comma => {
                    if is_parenthesis_head {
                        if head.is_empty() {
                            return Err((
                                "Unexpected comma in macro definition head, expected macro argument".to_string(),
                                ptkn.position.clone(),
                            ));
                        } 
                        
                        if has_comma {
                            return Err((
                                "Unexpected comma in macro definition head".to_string(),
                                ptkn.position.clone(),
                            ));
                        }

                        has_comma = true;
                        ptkns_found_quantity += 1;
                    } else {
                        return Ok((head, ptkns_found_quantity));
                    }
                }

                Token::MacroArgIdentifier(_) => {
                    if is_parenthesis_head {
                        
                        if head.is_empty() && has_comma {
                            return Err((
                                "Unexpected macro argument identifier in macro definition head, expected macro argument".to_string(),
                                ptkn.position.clone(),
                            ));
                        }

                        
                        // check if already exists (study this after)
                        for existing_ptkn in head.iter() {
                            if let Token::MacroArgIdentifier(existing_arg_name) = &existing_ptkn.token {
                                if let Token::MacroArgIdentifier(arg_name) = &ptkn.token {
                                    if existing_arg_name == arg_name {
                                        return Err((
                                            "Duplicate macro argument identifier detected".to_string(),
                                            ptkn.position.clone(),
                                        ));
                                    }
                                }
                            }
                        }
                        head.push(ptkn.clone());
                        ptkns_found_quantity += 1;
                        has_comma = false;
                    } else {
                        return Ok((head, ptkns_found_quantity));
                    }
                }

                _ => {
                    if is_parenthesis_head {
                        return Err((
                            "Unexpected expression in macro definition head, expected macro argument or comma".to_string(),
                            ptkn.position.clone(),
                        ));
                    } else {
                        return Ok((head, ptkns_found_quantity));
                    }
                }
            }

            ptoken_counter += 1;
            last_position = ptkn.position.clone();
        }

        if is_parenthesis_head && has_comma {
            return Err((
                "Unexpected end of macro definition head, expected macro argument".to_string(),
                last_position,
            ));
        }

        Ok((head, ptkns_found_quantity))
    }

    fn scan_macro_sequence(&self, start_index: usize, identifier_line: u32) -> Result<(Vec<PositionedToken>, usize), (String, Position)> {
        // This function reads a sequence of tokens that defines a define processor
        // Backslash ables to continue reading the sequence in the next line
        let mut sequence: Vec<PositionedToken> = Vec::new();
        let mut ptokens_read: usize = 0;
        let mut line_to_read: u32 = identifier_line;
        let mut index = start_index;
        
        while index < self.len() {
            let ptk = match self.get(index) {
                Some(ptk) => ptk,
                None => break,
            };

            if ptk.position.line > line_to_read {
                break;
            } else if ptk.position.line < line_to_read {
                return Err( ("Unexpected line change in define sequence".to_string(), ptk.position) );
            }

            match ptk.token.clone() {
                Token::Backslash => {
                    // If the token is a backslash, we continue reading in the next line
                    line_to_read += 1;
                    ptokens_read += 1;
                    index += 1;
                    continue;
                }
                _ => {
                    sequence.push(ptk.clone());
                    ptokens_read += 1;
                    index += 1;
                    continue;
                }
            }
        
        }

        Ok((sequence, ptokens_read))

    }

    fn scan_macro_calling_head(
        &self,
        vector_offset: usize,
        identifier_position: Position,
    ) -> Result<(Vec<PositionedToken>, usize), (String, Position)> {
        /* 
            the same as scan_macro_definition_head, but the arguments can be any type
            of token
        */
        /* e.g:
            macro(0, 2, "hi", $carry) returns (a vector of [0,2,"hi",$carry], the quantity of tokens found
            macro() returns (empty vector, 2)
            macro(1, 2,3,) -> cause error
            macro(,) -> cause error
            macro(1 2,) -> cause error
            macro(1,2 3) -> cause error
         */
        let mut head: Vec<PositionedToken> = Vec::new();
        let mut ptkns_found_quantity = 0;

        let mut is_parenthesis_head = false;
        let mut ptoken_counter = vector_offset;
        let mut last_position = identifier_position.clone();
        let mut has_comma = false;

        while ptoken_counter < self.len() {
            let ptkn = match self.get(ptoken_counter) {
            Some(ptkn) => ptkn,
            None => break,
            };

            match ptkn.token.clone() {
            Token::LeftParenthesis => {
                if !is_parenthesis_head {
                is_parenthesis_head = true;
                ptkns_found_quantity += 1;
                } else {
                return Err((
                    "Unexpected left parenthesis in macro calling head".to_string(),
                    ptkn.position.clone(),
                ));
                }
            }

            Token::RightParenthesis => {
                if !is_parenthesis_head {
                return Err((
                    "Unexpected right parenthesis in macro calling head".to_string(),
                    ptkn.position.clone(),
                ));
                }
                if has_comma {
                return Err((
                    "Unexpected right parenthesis in macro calling head, expected argument".to_string(),
                    ptkn.position.clone(),
                ));
                } else {
                ptkns_found_quantity += 1;
                return Ok((head, ptkns_found_quantity));
                }
            }
            Token::Comma => {
                if is_parenthesis_head {
                if head.is_empty() {
                    return Err((
                    "Unexpected comma in macro calling head, expected argument".to_string(),
                    ptkn.position.clone(),
                    ));
                }

                if has_comma {
                    return Err((
                    "Unexpected comma in macro calling head".to_string(),
                    ptkn.position.clone(),
                    ));
                }

                has_comma = true;
                ptkns_found_quantity += 1;
                } else {
                return Ok((head, ptkns_found_quantity));
                }
            }

            _ => {
                if is_parenthesis_head {
                if has_comma || head.is_empty() {
                    head.push(ptkn.clone());
                    ptkns_found_quantity += 1;
                    has_comma = false;
                } else {
                    return Err((
                    "Unexpected token in macro calling head, expected comma".to_string(),
                    ptkn.position.clone(),
                    ));
                }
                } else {
                return Ok((head, ptkns_found_quantity));
                }
            }
            }

            ptoken_counter += 1;
            last_position = ptkn.position.clone();
        }

        if is_parenthesis_head && has_comma {
            return Err((
            "Unexpected end of macro calling head, expected argument".to_string(),
            last_position,
            ));
        }

        Ok((head, ptkns_found_quantity))
    }
}