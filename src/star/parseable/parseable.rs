use crate::star::debuggable::Debugable;
use crate::star::utils::*;
use crate::star::core::*;
use super::ast::*;
use super::reader::*;
use super::sequence::*;

pub trait Parseable {
    fn parse(&self, ptokens: &Vec<PositionedToken>) -> Ast;
    fn read_comma_separated_numbers(&self, ptokens: &Vec<PositionedToken>, start_index: usize) -> Vec<PositionedToken>;
}

impl Parseable for Star {
    fn parse(&self, ptokens: &Vec<PositionedToken>) -> Ast {
        let mut ast = Ast {
            data_field: Vec::new(),
            instr_field: Vec::new(),
        };

        let mut field: Directive = Directive::Instr;
        let mut label_declaration_accumulator: Vec<PositionedToken> = Vec::new();

        let mut ptk_counter = 0;
        while ptk_counter < ptokens.len() {
            let ptk = match ptokens.get(ptk_counter) {
                Some(ptk) => ptk,
                None => unreachable!(),
            };
            match ptk.token {
                // ==== Detect the current field ====
                Token::Directive(Directive::Data) => {
                    field = Directive::Data;
                    ptk_counter += 1;
                    continue;
                }
                Token::Directive(Directive::Instr) => {
                    field = Directive::Instr;
                    ptk_counter += 1;
                    continue;
                }
                // ==== Case not is a Data or Instr Directive ====
                _ => {
                    match field {
                        // ==== Data Field ====
                        Directive::Data => {
                            match ptk.token {
                                Token::LabelDeclaration(_) => {
                                    label_declaration_accumulator.push(ptk.clone());
                                    ptk_counter += 1;
                                    continue;
                                }
                                Token::Directive(Directive::Byte)
                                | Token::Directive(Directive::Word) => {
                                    let data: Vec<PositionedToken> = self.read_comma_separated_numbers(ptokens, ptk_counter + 1);
                                    let data_len = data.len();
                                    if data_len <= 0 {
                                        self.exit_with_positional_error(
                                            "Directive expects at least one number",
                                            ptk.position,
                                        )
                                    }
                                    ast.data_field.push(
                                        DataCamp {
                                            label_declarations: label_declaration_accumulator.clone(),
                                            directive: ptk.clone(),
                                            arg: DataCampArg::Multiple(data.clone()),
                                        }
                                    );
                                    ptk_counter += data_len * 2;
                                    label_declaration_accumulator.clear();
                                    continue;
                                }
                                Token::Directive(Directive::Space) => {
                                    match ptokens.get(ptk_counter + 1) {
                                        Some(next_ptk) => {
                                            if let Token::NumberLiteral(_) = next_ptk.token {
                                                ast.data_field.push(
                                                    DataCamp {
                                                        label_declarations: label_declaration_accumulator.clone(),
                                                        directive: ptk.clone(),
                                                        arg: DataCampArg::Unique(next_ptk.clone()),
                                                    }
                                                );
                                                ptk_counter += 2;
                                                label_declaration_accumulator.clear();
                                                continue;
                                            } else {
                                                self.exit_with_positional_error(
                                                    "Directive expects a number",
                                                    ptk.position,
                                                );
                                            }
                                        }
                                        None => self.exit_with_positional_error(
                                            "Directive expects a number",
                                            ptk.position,
                                        ),
                                    };
                                }
                                Token::Directive(Directive::String)
                                | Token::Directive(Directive::Stringz) => {
                                    match ptokens.get(ptk_counter + 1) {
                                        Some(next_ptk) => {
                                            if let Token::StringLiteral(_) = next_ptk.token {
                                                ast.data_field.push(
                                                    DataCamp {
                                                        label_declarations: label_declaration_accumulator.clone(),
                                                        directive: ptk.clone(),
                                                        arg: DataCampArg::Unique(next_ptk.clone()),
                                                    }
                                                );
                                                ptk_counter += 2;
                                                label_declaration_accumulator.clear();
                                                continue;
                                            } else {
                                                self.exit_with_positional_error(
                                                    "Directive expects a literal string",
                                                    ptk.position,
                                                );
                                            }
                                        }
                                        None => self.exit_with_positional_error(
                                            "Directive expects a literal string",
                                            ptk.position,
                                        ),
                                    };
                                }
                                Token::Directive(Directive::Checkpoint) => {
                                    ast.data_field.push(
                                        DataCamp {
                                            label_declarations: label_declaration_accumulator.clone(),
                                            directive: ptk.clone(),
                                            arg: DataCampArg::Empty,
                                        }
                                    );
                                    ptk_counter += 1;
                                    label_declaration_accumulator.clear();
                                    continue;
                                }
                                _ => self.exit_with_positional_error(
                                    "Invalid expression in data field",
                                    ptk.position,
                                ),
                            }
                        }

                        // ==== Instruction Field ====
                        Directive::Instr => {
                            match ptk.token.clone() {
                                // ==== Label Declaration ====
                                Token::LabelDeclaration(_) => {
                                    label_declaration_accumulator.push(ptk.clone());
                                    ptk_counter += 1;
                                    continue;
                                }
                                Token::Instruction(instr) => {
                                    // ==== READ NONE ====
                                    match instr {
                                        Instruction::Mcall
                                        => {
                                            // e.g.: ret
                                            ast.instr_field.push(
                                                InstrCamp {
                                                    label_declarations: label_declaration_accumulator.clone(),
                                                    instruction: ptk.clone(),
                                                    sequence: Sequence::Zero,
                                                }
                                            );
                                            ptk_counter += 1;
                                            label_declaration_accumulator.clear();
                                            continue;
                                        }

                                        // ==== READ REG ====
                                        Instruction::J => {
                                            // e.g.: j $r
                                            match read_r_sequence(&ptokens, ptk_counter + 1, ptk.position) {
                                                Ok(sequence) => {
                                                    ast.instr_field.push(
                                                        InstrCamp {
                                                            label_declarations: label_declaration_accumulator.clone(),
                                                            instruction: ptk.clone(),
                                                            sequence,
                                                        }
                                                    );
                                                    ptk_counter += 2;
                                                    label_declaration_accumulator.clear();
                                                    continue;
                                                }
                                                Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                            }
                                        }
                                        /* DELETED:
                                            Instruction::Br => {
                                                // e.g.: br $r
                                                match read_r_sequence(&ptokens, ptk_counter + 1, ptk.position) {
                                                    Ok(sequence) => {
                                                        ast.instr_field.push(
                                                            InstrCamp {
                                                                label_declarations: label_declaration_accumulator.clone(),
                                                                instruction: ptk.clone(),
                                                                sequence,
                                                            }
                                                        );
                                                        ptk_counter += 2;
                                                        label_declaration_accumulator.clear();
                                                        continue;
                                                    }
                                                    Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                                }
                                            }
                                        */
                                        // ==== READ REG REG ====
                                        Instruction::Xlb

                                        | Instruction::Lab
                                        | Instruction::Llb
                                        | Instruction::Sab
                                        | Instruction::Slb
                                        | Instruction::Mulhl
                                        | Instruction::Divhl
                                        | Instruction::Muluhl
                                        | Instruction::Divuhl
                                        | Instruction::Not
                                        => {
                                            // e.g.: xlb $r1, $r2
                                            match read_r_r_sequence(&ptokens, ptk_counter + 1, ptk.position) {
                                                Ok(sequence) => {
                                                    ast.instr_field.push(
                                                        InstrCamp {
                                                            label_declarations: label_declaration_accumulator.clone(),
                                                            instruction: ptk.clone(),
                                                            sequence,
                                                        }
                                                    );
                                                    ptk_counter += 4;
                                                    label_declaration_accumulator.clear();
                                                    continue;
                                                }
                                                Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                            }
                                        }

                                        // ==== READ REG IMM ====
                                        Instruction::Lai
                                        | Instruction::Lli
                                        => {
                                            // e.g.: lai $r, $imm<8>
                                            match read_r_n_sequence(&ptokens, ptk_counter + 1, ptk.position) {
                                                Ok(sequence) => {
                                                    ast.instr_field.push(
                                                        InstrCamp {
                                                            label_declarations: label_declaration_accumulator.clone(),
                                                            instruction: ptk.clone(),
                                                            sequence,
                                                        }
                                                    );
                                                    ptk_counter += 4;
                                                    label_declaration_accumulator.clear();
                                                    continue;
                                                }
                                                Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                            } 
                                        }

                                        // ==== READ REG REG REG ====
                                        Instruction::Add
                                        | Instruction::Sub
                                        | Instruction::And
                                        | Instruction::Or
                                        | Instruction::Xor
                                        | Instruction::Shl
                                        | Instruction::Shr
                                        | Instruction::Beqr
                                        | Instruction::Bneqr
                                        | Instruction::Bgtr
                                        | Instruction::Bltr
                                        | Instruction::Bgtur
                                        | Instruction::Bltur
                                        => {
                                            // e.g.: add $r, $r, $r
                                            match read_r_r_r_sequence(&ptokens, ptk_counter + 1, ptk.position) {
                                                Ok(three_seq) => {
                                                    ast.instr_field.push(
                                                        InstrCamp {
                                                            label_declarations: label_declaration_accumulator.clone(),
                                                            instruction: ptk.clone(),
                                                            sequence: three_seq,
                                                        }
                                                    );
                                                    ptk_counter += 6;
                                                    label_declaration_accumulator.clear();
                                                    continue;
                                                }
                                                Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                            }
                                        }
                                    }
                                }
                                Token::PseudoInstruction(pseudo_instr) => {
                                    match pseudo_instr {
                                        PseudoInstruction::Nope
                                        | PseudoInstruction::Ret
                                        => {
                                            // e.g.: nope
                                            ast.instr_field.push(
                                                InstrCamp {
                                                    label_declarations: label_declaration_accumulator.clone(),
                                                    instruction: ptk.clone(),
                                                    sequence: Sequence::Zero,
                                                }
                                            );
                                            ptk_counter += 1;
                                            label_declaration_accumulator.clear();
                                            continue;
                                        }
                                        
                                        PseudoInstruction::Inc
                                        | PseudoInstruction::Dec

                                        | PseudoInstruction::Jr
                                        => {
                                            // e.g.: jr $r
                                            match read_r_sequence(&ptokens, ptk_counter + 1, ptk.position) {
                                                Ok(sequence) => {
                                                    ast.instr_field.push(
                                                        InstrCamp {
                                                            label_declarations: label_declaration_accumulator.clone(),
                                                            instruction: ptk.clone(),
                                                            sequence,
                                                        }
                                                    );
                                                    ptk_counter += 2;
                                                    label_declaration_accumulator.clear();
                                                    continue;
                                                }
                                                Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                            }
                                        }
                                        // ==== READ IDENTIFIER ====
                                        PseudoInstruction::Ja => {
                                            // e.g.: ba address
                                            match read_id_sequence(&ptokens, ptk_counter + 1, ptk.position) {
                                                Ok(sequence) => {
                                                    ast.instr_field.push(
                                                        InstrCamp {
                                                            label_declarations: label_declaration_accumulator.clone(),
                                                            instruction: ptk.clone(),
                                                            sequence,
                                                        }
                                                    );
                                                    ptk_counter += 2;
                                                    label_declaration_accumulator.clear();
                                                    continue;
                                                }
                                                Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                            }
                                        }
                                        // ==== READ REG REG ====
                                        PseudoInstruction::Move
                                        | PseudoInstruction::Swap
                                        => {
                                            match read_r_r_sequence(&ptokens, ptk_counter + 1, ptk.position) {
                                                Ok(sequence) => {
                                                    ast.instr_field.push(
                                                        InstrCamp {
                                                            label_declarations: label_declaration_accumulator.clone(),
                                                            instruction: ptk.clone(),
                                                            sequence,
                                                        }
                                                    );
                                                    ptk_counter += 4;
                                                    label_declaration_accumulator.clear();
                                                    continue;
                                                }
                                                Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                            }
                                        }
                                        
                                        PseudoInstruction::Li
                                        => {
                                            // e.g.: li $r, $imm
                                            match read_r_n_sequence(&ptokens, ptk_counter + 1, ptk.position) {
                                                Ok(sequence) => {
                                                    ast.instr_field.push(
                                                        InstrCamp {
                                                            label_declarations: label_declaration_accumulator.clone(),
                                                            instruction: ptk.clone(),
                                                            sequence,
                                                        }
                                                    );
                                                    ptk_counter += 4;
                                                    label_declaration_accumulator.clear();
                                                    continue;
                                                }
                                                Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                            }
                                        }

                                        // ==== READ REG IDENTIFIER ====
                                        PseudoInstruction::La
                                        => {
                                            // e.g.: la $r, address
                                            match read_r_id_sequence(&ptokens, ptk_counter + 1, ptk.position) {
                                                Ok(sequence) => {
                                                    ast.instr_field.push(
                                                        InstrCamp {
                                                            label_declarations: label_declaration_accumulator.clone(),
                                                            instruction: ptk.clone(),
                                                            sequence,
                                                        }
                                                    );
                                                    ptk_counter += 4;
                                                    label_declaration_accumulator.clear();
                                                    continue;
                                                }
                                                Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                            }
                                        }

                                        // ==== READ REG REG REG ====
                                        PseudoInstruction::Mul
                                        | PseudoInstruction::Div
                                        | PseudoInstruction::Mod
                                        => {
                                            // e.g.: mul $rd, $rs, $rt
                                            match read_r_r_r_sequence(&ptokens, ptk_counter + 1, ptk.position) {
                                                Ok(three_seq) => {
                                                    ast.instr_field.push(
                                                        InstrCamp {
                                                            label_declarations: label_declaration_accumulator.clone(),
                                                            instruction: ptk.clone(),
                                                            sequence: three_seq,
                                                        }
                                                    );
                                                    ptk_counter += 6;
                                                    label_declaration_accumulator.clear();
                                                    continue;
                                                }
                                                Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                            }
                                        }

                                        // ==== READ REG REG NUMBER ====
                                        PseudoInstruction::Addi
                                        | PseudoInstruction::Subi
                                        | PseudoInstruction::Andi
                                        | PseudoInstruction::Ori
                                        | PseudoInstruction::Xori
                                        | PseudoInstruction::Shli
                                        | PseudoInstruction::Shri
                                        | PseudoInstruction::Muli
                                        | PseudoInstruction::Divi
                                        | PseudoInstruction::Modi
                                        => {
                                            // e.g.: addi $rd, $rs, imm
                                            match read_r_r_n_sequence(&ptokens, ptk_counter + 1, ptk.position) {
                                                Ok(sequence) => {
                                                    ast.instr_field.push(
                                                        InstrCamp {
                                                            label_declarations: label_declaration_accumulator.clone(),
                                                            instruction: ptk.clone(),
                                                            sequence,
                                                        }
                                                    );
                                                    ptk_counter += 6;
                                                    label_declaration_accumulator.clear();
                                                    continue;
                                                }
                                                Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                            }
                                        }

                                        // READ REG REG SBRACKET NUMBER SBRACKET
                                        PseudoInstruction::Lb
                                        | PseudoInstruction::Lw
                                        | PseudoInstruction::Sb
                                        | PseudoInstruction::Sw
                                        => {
                                            // e.g.: lb $rd, $rs[imm]
                                            match read_r_r_br_n_br(&ptokens, ptk_counter + 1, ptk.position) {
                                                Ok(sequence) => {
                                                    ast.instr_field.push(
                                                        InstrCamp {
                                                            label_declarations: label_declaration_accumulator.clone(),
                                                            instruction: ptk.clone(),
                                                            sequence,
                                                        }
                                                    );
                                                    ptk_counter += 7;
                                                    label_declaration_accumulator.clear();
                                                    continue;
                                                }
                                                Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                            }
                                        }

                                        // ==== READ REG REG IDENTIFIER ====
                                        PseudoInstruction::Beqa
                                        | PseudoInstruction::Bneqa
                                        | PseudoInstruction::Bgta
                                        | PseudoInstruction::Blta
                                        | PseudoInstruction::Bgtua
                                        | PseudoInstruction::Bltua
                                        => {
                                            // e.g.: beqa $rs, $rt, address
                                            match read_r_r_id_sequence(&ptokens, ptk_counter + 1, ptk.position) {
                                                Ok(sequence) => {
                                                    ast.instr_field.push(
                                                        InstrCamp {
                                                            label_declarations: label_declaration_accumulator.clone(),
                                                            instruction: ptk.clone(),
                                                            sequence,
                                                        }
                                                    );
                                                    ptk_counter += 6;
                                                    label_declaration_accumulator.clear();
                                                    continue;
                                                }
                                                Err((err_msg, err_pos)) => self.exit_with_positional_error(&err_msg, err_pos),
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    self.exit_with_positional_error(
                                        "Invalid expression in instruction field",
                                        ptk.position,
                                    );
                                }
                            }
                        }
                        _ => unreachable!()
                    }
                }
            }
        }

        ast
    }

    fn read_comma_separated_numbers(&self, ptokens: &Vec<PositionedToken>, start_index: usize) -> Vec<PositionedToken> {
        let mut numbers: Vec<PositionedToken> = Vec::new();
        let mut index: usize = start_index;

        while index < ptokens.len() {
            let ptk = match ptokens.get(index) {
                Some(ptk) => ptk,
                None => break,
            };
            match ptk.token {
                Token::NumberLiteral(_) => {
                    numbers.push(ptk.clone());
                    index += 1;
                }
                Token::Comma => {
                    index += 1;
                }
                _ => break,
            }
        }

        numbers
    }
}


