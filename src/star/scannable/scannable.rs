use std::collections::{HashMap, HashSet};

use crate::debugger;
use crate::star::core::*;
use crate::star::utils::*;
use crate::star::debuggable::*;
use crate::star::scannable::positioned_tokens_vectorable::*;

type MacroTable = HashMap<String, (Vec<PositionedToken>, Vec<PositionedToken>)>;

pub trait Scannable {
    fn scan(&mut self, base_file_path: &String) -> Vec<PositionedToken>;

    fn scan_and_resolve_processors(
        &mut self,
        including_file_path: &String,
        file_path_to_include: &String,
        file_counter: &mut u32,
        file_dependency_table: &mut HashMap<u32, HashSet<u32>>,
        macro_table: &mut MacroTable,
        once_set: &mut HashSet<String>,
        processing_stack: &mut HashSet<u32>,
    ) -> Vec<PositionedToken>;
}

impl Scannable for Star {
    fn scan(&mut self, base_file_path: &String) -> Vec<PositionedToken> {
        let mut file_dependency_table: HashMap<u32, HashSet<u32>> = HashMap::new();
        let mut macro_table: MacroTable = HashMap::new();
        let mut once_set: HashSet<String> = HashSet::new();
        let mut file_counter: u32 = 0;
        let mut processing_stack: HashSet<u32> = HashSet::new();

        let ptkns = self.scan_and_resolve_processors(
            &"No one".to_string(),
            base_file_path,
            &mut file_counter,
            &mut file_dependency_table,
            &mut macro_table,
            &mut once_set,
            &mut processing_stack,
        );
        ptkns
    }

    fn scan_and_resolve_processors(
        &mut self,
        including_file_path: &String,
        file_path_to_include: &String,
        file_counter: &mut u32,
        file_dependency_table: &mut HashMap<u32, HashSet<u32>>,
        macro_table: &mut MacroTable,
        once_set: &mut HashSet<String>,
        processing_stack: &mut HashSet<u32>,
    ) -> Vec<PositionedToken> {
        // ==== GETTING THE ABSOLUTE FILE PATH STRING ====
        use std::fs;
        let absolute_file_path: String = match fs::canonicalize(file_path_to_include) {
            Ok(path) => path.to_str().unwrap_or(file_path_to_include).to_string(),
            Err(_) => {
                debugger::exit_with_error(&format!(
                    "The file {} does not exist or could not be read",
                    file_path_to_include
                ));
                unreachable!();
            }
        };
        
        // ==== CHECKING IF THE FILE IS ALREADY SCANNED ====
        let file_id = match self.get_file_id_by_path(&absolute_file_path) {
            Some(id) => id,
            None => {
                *file_counter += 1;
                *file_counter
            }
        };

        // ==== CHECKING FOR IMPORT CYCLES ====
        if processing_stack.contains(&file_id) {
            debugger::exit_with_error(
                &format!(
                    "Include cycle detected: [{} -> {}]",
                    including_file_path.beautiful_path(),
                    absolute_file_path.beautiful_path(),
                )
            );
        }
        processing_stack.insert(file_id);

        // ==== SCANNING THE FILE CONTENT ====
        let mut ptokens: Vec<PositionedToken> = match scan_positioned_tokens_from_file(&absolute_file_path, file_id) {
            Ok(tkns) => tkns,
            Err((err, position_option)) => {
                match position_option {
                    Some(position) => self.exit_with_positional_error(&err, position),
                    None => debugger::exit_with_error(&err),
                }
                return Vec::new();
            }
        };

        // ==== ADDING THE FILE TO THE FILE TABLE ====
        if !self.file_table.contains_key(&file_id) {
            self.file_table.insert(file_id, absolute_file_path.clone());
        }

        // ==== RESOLVING INCLUDES ====
        let mut token_counter: usize = 0;
        let mut ptokens_len: usize = ptokens.len();

        while token_counter < ptokens_len {
            let tk = match ptokens.get(token_counter) {
                Some(tk) => tk,
                None => break,
            };
            
            match tk.token.clone() {
                Token::Processor(Processor::Include) => {
                    match ptokens.get(token_counter + 1).cloned() {
                        Some(next_p_tkn) => {
                            if let Token::StringLiteral(include_path_literal_string) = next_p_tkn.token.clone() {
                                
                                let included_ptokens = self.scan_and_resolve_processors(
                                    &absolute_file_path,
                                    &include_path_literal_string,
                                    file_counter,
                                    file_dependency_table,
                                    macro_table,
                                    once_set,
                                    processing_stack,
                                );
                                match file_dependency_table.get_mut(&file_id) {
                                    Some(dependencies) => {
                                        dependencies.insert(*file_counter);
                                    }
                                    None => {
                                        file_dependency_table.insert(file_id, HashSet::from([*file_counter]));
                                    }
                                }

                                ptokens.remove(token_counter); // Remove the @include token
                                ptokens.remove(token_counter); // Remove the file_path token

                                // Insert the included tokens at the current position
                                for included_ptkn in included_ptokens.into_iter().rev() {
                                    ptokens.insert(token_counter, included_ptkn);
                                }
                                ptokens_len = ptokens.len();
                            } else {
                                self.exit_with_positional_error(
                                    "Include directive must be followed by a file path",
                                    next_p_tkn.position,
                                );
                            }
                        }
                        None => {
                            self.exit_with_positional_error(
                                "Include directive must be followed by a file path",
                                tk.position,
                            );
                        }
                    }
                }
                Token::Processor(Processor::Define) => {
                    match ptokens.get(token_counter + 1).cloned() {
                        Some( define_identifier_ptkn ) => {
                            match define_identifier_ptkn.token {
                                Token::Identifier(identifier_string) => {
                                    let (macro_head, macro_definition_head_tkns_found) = match ptokens.scan_macro_definition_head(token_counter + 2, define_identifier_ptkn.position) {
                                        
                                        Ok((head, ptkns_found)) => (head, ptkns_found),
                                        Err((e, error_pos)) => {
                                            self.exit_with_positional_error(&e, error_pos);
                                            unreachable!();
                                        }
                                    };
                                    match ptokens.scan_macro_sequence(token_counter + macro_definition_head_tkns_found + 2, define_identifier_ptkn.position.line) {
                                        Ok((define_sequence, ptokens_sequence_quantity_found)) => {
                                            macro_table.insert(
                                                identifier_string,
                                                (macro_head, define_sequence.clone()),
                                            );
                                            // remove the define, identifier, head and sequence tokens
                                            for _ in 0..(ptokens_sequence_quantity_found + macro_definition_head_tkns_found + 2) {
                                                if ptokens.len() > token_counter {
                                                    ptokens.remove(token_counter);
                                                }
                                            }
                                            ptokens_len = ptokens.len();
                                            continue;
                                        }
                                        Err((err, err_pos)) => {
                                            self.exit_with_positional_error(&err, err_pos);
                                        }
                                    }
                                }
                                _ => {
                                    self.exit_with_positional_error(
                                        "Define directive must be followed by an identifier",
                                        define_identifier_ptkn.position,
                                    );
                                }
                            }
                        }
                        None => 
                            self.exit_with_positional_error(
                                "Define directive must be followed by an identifier",
                                tk.position,
                            ),
                    }
                }
                Token::Processor(Processor::Once) => {
                    match once_set.get(&absolute_file_path) {
                        Some(_) => {
                            // remove all forward
                            while token_counter < ptokens.len() {
                                ptokens.remove(token_counter);
                            }
                        }
                        None => {
                            once_set.insert(absolute_file_path.clone());
                            ptokens.remove(token_counter);
                            ptokens_len = ptokens.len();
                        }
                    }
                }
                Token::Identifier(identifier_string) => {
                    // Check if the identifier is a defined processor
                    if let Some((macro_head, define_sequence)) = macro_table.get(&identifier_string) {
                        let (macro_call_head, head_tokens_quantity_found) = match ptokens.scan_macro_calling_head(token_counter+1, tk.position) {
                            Ok((head, ptkns_to_skip)) => (head, ptkns_to_skip),
                            Err((e, error_pos)) => {
                                self.exit_with_positional_error(&e, error_pos);
                                unreachable!();
                            }
                        };

                        if macro_call_head.len() != macro_head.len() {
                            self.exit_with_positional_error(
                                "Macro called with incorrect number of arguments",
                                tk.position,
                            );
                        }

                        let mut ptkns_to_substitute = define_sequence.clone();

                        for (i, macro_call_arg) in macro_call_head.iter().enumerate() {
                            let definition_arg_string = match macro_head.get(i) {
                                Some(arg) => {
                                    if let Token::MacroArgIdentifier(arg_name) = arg.token.clone() {
                                        arg_name
                                    } else {
                                        self.exit_with_positional_error(
                                            "Expected macro argument",
                                            arg.position,
                                        );
                                        unreachable!();
                                    }
                                }
                                None => {
                                    self.exit_with_positional_error(
                                        &format!("Macro '{}' called with too many arguments", identifier_string),
                                        tk.position,
                                    );
                                    unreachable!();
                                }
                            };

                            for df_sq_ptkn in ptkns_to_substitute.iter_mut() {
                                if let Token::MacroArgIdentifier(df_sq_macro_arg_name) = df_sq_ptkn.token.clone() {
                                    if df_sq_macro_arg_name == definition_arg_string {
                                        *df_sq_ptkn = macro_call_arg.clone();
                                    }
                                }
                            }
                        }

                        // check if there are not substituted macro arguments
                        for df_sq_ptkn in ptkns_to_substitute.iter() {
                            if let Token::MacroArgIdentifier(df_sq_macro_arg_name) = df_sq_ptkn.token.clone() {
                                self.exit_with_positional_error(
                                    &format!("Macro argument '{}' not associated with any argument in the macro definition head", df_sq_macro_arg_name), 
                                    df_sq_ptkn.position,
                                );
                            }
                        }
                        // remove the identifier token and the head
                        let tk_q_to_rem = 1 + head_tokens_quantity_found;
                        
                        for _ in 0..(tk_q_to_rem) {
                            if ptokens.len() > token_counter {
                                ptokens.remove(token_counter);
                            }
                        }

                        // Insert the defined processor tokens at the current position
                        for def_ptk in ptkns_to_substitute.iter().rev() {
                            ptokens.insert(token_counter, def_ptk.clone());
                        }
                        ptokens_len = ptokens.len();
                        continue;
                    } else {
                        token_counter += 1;
                        continue;
                    }
                }
                _ => {
                    token_counter += 1;
                    continue;
                }
            }
        }

        processing_stack.remove(&file_id);
        return ptokens;
    }
}

fn read_file_content(file_path: &String) -> Result<String, String> {
    use std::fs;

    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content.replace("\r", "")), // Normalize line endings
        Err(_) => Err(format!("The file {} does not exist or could not be read", file_path)),
    }
}

fn scan_positioned_tokens_from_file(file_path: &String, file_id: u32) -> Result<Vec<PositionedToken>, (String, Option<Position>)> {
    match read_file_content(file_path) {
        Ok(content) => {
            let mut tokens: Vec<PositionedToken> = Vec::new();
            let mut token_accumulator = String::new();

            let mut actual_line = 1;
            let mut actual_column = 1;

            let mut initial_token_column = 1;

            let mut is_string_literal_mode = false;
            let mut is_commentary = false;
            let mut line_has_identation = false;

            let mut chars = content.chars().peekable();
            while let Some(ch) = chars.next() {
                if token_accumulator.is_empty() {
                    initial_token_column = actual_column;
                }
                match ch {
                    '\t' => {
                        if !is_commentary && !is_string_literal_mode {
                            line_has_identation = true;
                        }
                        actual_column += 1;
                        continue;
                    }
                    '\n' => {
                        // Finaliza comentário e reseta estados
                        is_commentary = false;
                        line_has_identation = false;

                        // Adiciona token acumulado, se houver
                        if !token_accumulator.is_empty() && !is_string_literal_mode {
                            match tokens.push_positioned_token(
                                token_accumulator.clone(),
                                file_id,
                                actual_line,
                                if !line_has_identation { Some(initial_token_column) } else { None },
                            ) {
                                Ok(_) => {}
                                Err(e) => {
                                    return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                                }
                            }
                            token_accumulator.clear();
                        }

                        actual_line += 1;
                        actual_column = 1;
                        continue;
                    }
                    '#' => {
                        if !is_string_literal_mode {
                            // Inicia modo de comentário
                            is_commentary = true;
                            // Adiciona token acumulado, se houver
                            if !token_accumulator.is_empty() {
                                match tokens.push_positioned_token(
                                    token_accumulator.clone(),
                                    file_id,
                                    actual_line,
                                    if !line_has_identation { Some(initial_token_column) } else { None },
                                ) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                                    }
                                }
                                token_accumulator.clear();
                            }
                        } else {
                            // Dentro de string literal, '#' é tratado como caractere comum
                            token_accumulator.push(ch);
                        }
                        actual_column += 1;
                        continue;
                    }
                    ' ' => {
                        if is_commentary || is_string_literal_mode {
                            if is_string_literal_mode {
                                token_accumulator.push(ch);
                            }
                            actual_column += 1;
                            continue;
                        }
                        // Espaço fora de comentário ou string delimita um token
                        if !token_accumulator.is_empty() {
                            match tokens.push_positioned_token(
                                token_accumulator.clone(),
                                file_id,
                                actual_line,
                                if !line_has_identation { Some(initial_token_column) } else { None },
                            ) {
                                Ok(_) => {}
                                Err(e) => {
                                    return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                                }
                            }
                            token_accumulator.clear();
                        }
                        actual_column += 1;
                        continue;
                    }
                    '"' => {
                        if is_commentary {
                            actual_column += 1;
                            continue;
                        }
                        if is_string_literal_mode {
                            // Fecha string literal
                            token_accumulator.push(ch);
                            match tokens.push_positioned_token(
                                token_accumulator.clone(),
                                file_id,
                                actual_line,
                                if !line_has_identation { Some(initial_token_column) } else { None },
                            ) {
                                Ok(_) => {}
                                Err(e) => {
                                    return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                                }
                            }
                            token_accumulator.clear();
                            is_string_literal_mode = false;
                        } else {
                            // Inicia string literal
                            if !token_accumulator.is_empty() {
                                match tokens.push_positioned_token(
                                    token_accumulator.clone(),
                                    file_id,
                                    actual_line,
                                    if !line_has_identation { Some(initial_token_column) } else { None },
                                ) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                                    }
                                }
                                token_accumulator.clear();
                            }
                            token_accumulator.push(ch);
                            is_string_literal_mode = true;
                        }
                        actual_column += 1;
                        continue;
                    }
                    ',' | '[' | ']' | '(' | ')' | '\\' => {
                        if is_commentary || is_string_literal_mode {
                            if is_string_literal_mode {
                                token_accumulator.push(ch);
                            }
                            actual_column += 1;
                            continue;
                        }
                        if !token_accumulator.is_empty() {
                            match tokens.push_positioned_token(
                                token_accumulator.clone(),
                                file_id,
                                actual_line,
                                if !line_has_identation { Some(initial_token_column) } else { None },
                            ) {
                                Ok(_) => {}
                                Err(e) => {
                                    return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                                }
                            }
                            token_accumulator.clear();
                        }
                        initial_token_column = actual_column;
                        match tokens.push_positioned_token(
                            ch.to_string(),
                            file_id,
                            actual_line,
                            if !line_has_identation { Some(initial_token_column) } else { None },
                        ) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                            }
                        }
                        actual_column += 1;
                        continue;
                    }
                    ':' => {
                        if is_commentary || is_string_literal_mode {
                            if is_string_literal_mode {
                                token_accumulator.push(ch);
                            }
                            actual_column += 1;
                            continue;
                        }

                        token_accumulator.push(ch);
                        actual_column += 1;

                        if !token_accumulator.is_empty() {
                            match tokens.push_positioned_token(
                                token_accumulator.clone(),
                                file_id,
                                actual_line,
                                if !line_has_identation { Some(initial_token_column) } else { None },
                            ) {
                                Ok(_) => {}
                                Err(e) => {
                                    return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                                }
                            }
                            token_accumulator.clear();
                        }
                    }
                    _ => {
                        if is_commentary {
                            actual_column += 1;
                            continue;
                        }
                        // Caractere genérico: acumula no token atual
                        token_accumulator.push(ch);
                        actual_column += 1;
                    }
                }
            }

            // Adiciona o último token acumulado, se houver
            if !token_accumulator.is_empty() && !is_string_literal_mode {
                match tokens.push_positioned_token(
                    token_accumulator.clone(),
                    file_id,
                    actual_line,
                    if !line_has_identation { Some(initial_token_column) } else { None },
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                    }
                }
            }

            Ok(tokens)
        }
        Err(err) => return Err((err, None)),
    }
}