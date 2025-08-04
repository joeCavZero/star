use std::collections::HashMap;

use crate::star::core::*;
use crate::star::debuggable::*;
use crate::star::math::*;
use crate::star::parseable::*;
use crate::star::symbolable::*;
use crate::star::utils::*;

pub type SymbolTable = HashMap<String, u16>;

pub trait Resolveable {
    fn resolve(&self, ast: &mut Ast)  -> SymbolTable;

    fn resolve_space(&self, ast: &mut Ast);
    
    fn resolve_pseudo_instructions(&self, ast: &mut Ast, symbol_table: &SymbolTable);
}

impl Resolveable for Star {
    fn resolve(&self, ast: &mut Ast) -> SymbolTable {
        self.resolve_space(ast);
        let symbol_table: SymbolTable = self.get_symbol_table(ast);
        self.resolve_pseudo_instructions(ast, &symbol_table);
        symbol_table
    }
    fn resolve_space(&self, ast: &mut Ast) {
        let mut instr_field_len = ast.instr_field.len();
        let mut instr_counter: usize = 0;
        while instr_counter < instr_field_len {
            let instr_camp = match ast.instr_field.get(instr_counter) {
                Some(camp) => camp,
                None => break,
            };
            let zero_reg = PositionedToken {
                token: Token::GeneralRegister(GeneralRegister::Zero),
                position: instr_camp.instruction.position,
            };
            let nope_camp = InstrCamp {
                label_declarations: Vec::new(),
                instruction: PositionedToken {
                    token: Token::Instruction(Instruction::Add),
                    position: instr_camp.instruction.position,
                },
                sequence: Sequence::Three(
                    zero_reg.clone(),
                    zero_reg.clone(),
                    zero_reg.clone(),
                ),
            };

            
            match instr_camp.instruction.token.clone() {
                Token::PseudoInstruction(pseudo_instruction) => {
                    match pseudo_instruction {
                        // ==== +0 ====
                        | PseudoInstruction::Nope
                        | PseudoInstruction::Move
                        | PseudoInstruction::Jr
                        | PseudoInstruction::Ret
                        => {}
                        // ==== +1 ====
                        PseudoInstruction::Li
                        | PseudoInstruction::La

                        | PseudoInstruction::Mul
                        | PseudoInstruction::Div
                        | PseudoInstruction::Mod
                        => {
                            ast.instr_field.insert(instr_counter + 1, nope_camp.clone());
                        }

                        // ==== +2 ====
                        PseudoInstruction::Swap

                        | PseudoInstruction::Addi
                        | PseudoInstruction::Subi
                        | PseudoInstruction::Andi
                        | PseudoInstruction::Ori
                        | PseudoInstruction::Xori
                        | PseudoInstruction::Shli
                        | PseudoInstruction::Shri

                        | PseudoInstruction::Beqa
                        | PseudoInstruction::Bneqa
                        | PseudoInstruction::Blta
                        | PseudoInstruction::Bgta
                        | PseudoInstruction::Bltua
                        | PseudoInstruction::Bgtua

                        | PseudoInstruction::Inc
                        | PseudoInstruction::Dec

                        | PseudoInstruction::Ja
                        => {
                            for _ in 0..2 {
                                ast.instr_field.insert(instr_counter + 1, nope_camp.clone());
                            }
                        }

                        // ==== +3 ====
                        PseudoInstruction::Sb

                        |PseudoInstruction::Muli
                        | PseudoInstruction::Divi
                        | PseudoInstruction::Modi
                        => {
                            for _ in 0..3 {
                                ast.instr_field.insert(instr_counter + 1, nope_camp.clone());
                            }
                        }

                        // ==== +4 ====
                        PseudoInstruction::Lb
                        => {
                            for _ in 0..4 {
                                ast.instr_field.insert(instr_counter + 1, nope_camp.clone());
                            }
                        }

                        // ==== +7 ====
                        PseudoInstruction::Lw
                        | PseudoInstruction::Sw
                        => {
                            for _ in 0..7 {
                                ast.instr_field.insert(instr_counter + 1, nope_camp.clone());
                            }
                        }

                    }
                    instr_counter += 1;
                    instr_field_len = ast.instr_field.len();
                    continue;
                }

                _ => {
                    instr_counter += 1;
                    instr_field_len = ast.instr_field.len();
                    continue;
                }
            }

        }
    }

    fn resolve_pseudo_instructions(&self, ast: &mut Ast, symbol_table: &SymbolTable) {
        let mut instr_counter: usize = 0;
        while instr_counter < ast.instr_field.len() {
            let instr_camp = match ast.instr_field.get_mut(instr_counter) {
                Some(camp) => camp,
                None => break,
            };
            let zero_reg = PositionedToken {
                token: Token::GeneralRegister(GeneralRegister::Zero),
                position: instr_camp.instruction.position,
            };
            let low_reg = PositionedToken {
                token: Token::GeneralRegister(GeneralRegister::Low),
                position: instr_camp.instruction.position,
            };
            let high_reg = PositionedToken {
                token: Token::GeneralRegister(GeneralRegister::High),
                position: instr_camp.instruction.position,
            };

            let aux1_reg = PositionedToken {
                token: Token::GeneralRegister(GeneralRegister::Aux1),
                position: instr_camp.instruction.position,
            };
            let aux2_reg = PositionedToken {
                token: Token::GeneralRegister(GeneralRegister::Aux2),
                position: instr_camp.instruction.position,
            };

            match instr_camp.instruction.token.clone() {
                Token::PseudoInstruction(pseudo_instruction) => {
                    
                    

                    match pseudo_instruction {
                        // >>>> 1 <<<<
                        // ==== NOPE ====
                        PseudoInstruction::Nope => {
                            instr_camp.instruction.token = Token::Instruction(Instruction::Add);
                            instr_camp.sequence = Sequence::Three(
                                zero_reg.clone(),
                                zero_reg.clone(),
                                zero_reg.clone(),
                            )
                        }
                        
                        // ==== MOVE ====
                        PseudoInstruction::Move => {
                            instr_camp.instruction.token = Token::Instruction(Instruction::Add);
                            if let Sequence::Two(arg1, arg2) = instr_camp.sequence.clone() {
                                instr_camp.sequence = Sequence::Three(
                                    arg1.clone(),
                                    zero_reg.clone(),
                                    arg2.clone(),
                                )
                            } else {
                                unreachable!()
                            }
                                
                        }

                        // ==== JUMP RELATIVE ====
                        PseudoInstruction::Jr => {
                            instr_camp.instruction.token = Token::Instruction(Instruction::Beqr);
                            if let Sequence::One(arg) = instr_camp.sequence.clone() {
                                instr_camp.sequence = Sequence::Three(
                                    zero_reg.clone(),
                                    zero_reg.clone(),
                                    arg.clone(),
                                );
                            } else {
                                unreachable!()
                            }
                        }

                        // ==== RETURN ====
                        PseudoInstruction::Ret => {
                            instr_camp.instruction.token = Token::Instruction(Instruction::J);
                            instr_camp.sequence = Sequence::One(
                                PositionedToken {
                                    token: Token::GeneralRegister(GeneralRegister::ReturnAddress),
                                    position: instr_camp.instruction.position.clone(),
                                },
                            );
                        }
                        // >>>> 2 <<<<
                        // ==== LOAD IMMEDIATE ====
                        PseudoInstruction::Li => {
                            if let Sequence::Two(reg_ptk, imm_ptk) = instr_camp.sequence.clone() {
                                if let Token::NumberLiteral(imm_string) = imm_ptk.token {
                                        
                                    let num = match u16_from_string(imm_string) {
                                        Ok(n) => n,
                                        Err(e) => {
                                            self.exit_with_positional_error(
                                                e.as_str(),
                                                imm_ptk.position.clone(),
                                            );
                                            unreachable!()
                                        }
                                    };
                                    let (num_low, num_high) = split_u16_to_strings(num);

                                    let new_camp_1 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken { 
                                            token: Token::Instruction( Instruction::Lli ),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            reg_ptk.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(num_low),
                                                position: imm_ptk.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_2 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken { 
                                            token: Token::Instruction( Instruction::Lai ),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            reg_ptk.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(num_high),
                                                position: imm_ptk.position.clone(),
                                            }
                                        ),
                                    };

                                    ast.instr_field[instr_counter] = new_camp_1;
                                    ast.instr_field[instr_counter + 1] = new_camp_2;
                                    
                                }
                            } else {
                                unreachable!()
                            }
                        }
                        
                        // ==== LOAD ADDRESS ====
                        PseudoInstruction::La => {
                            if let Sequence::Two(reg_ptk, address_ptk) = instr_camp.sequence.clone() {
                                if let Token::Identifier(address_string) = address_ptk.token {
                                        
                                    let num = match symbol_table.get(&address_string) {
                                        Some(n) => *n,
                                        None => {
                                            self.exit_with_positional_error(
                                                "Address not found",
                                                address_ptk.position.clone(),
                                            );
                                            unreachable!()
                                        }
                                    };
                                    let (num_low, num_high) = split_u16_to_strings(num);

                                    let new_camp_1 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken { 
                                            token: Token::Instruction( Instruction::Lli ),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            reg_ptk.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(num_low),
                                                position: address_ptk.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_2 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken { 
                                            token: Token::Instruction( Instruction::Lai ),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            reg_ptk.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(num_high),
                                                position: address_ptk.position.clone(),
                                            }
                                        ),
                                    };

                                    ast.instr_field[instr_counter] = new_camp_1;
                                    ast.instr_field[instr_counter + 1] = new_camp_2;
                                    
                                }
                            } else {
                                unreachable!()
                            }
                        }
                        
                        // ==== MULTIPLY ====
                        PseudoInstruction::Mul => {
                            if let Sequence::Three(arg1, arg2, arg3) = instr_camp.sequence.clone() {
                                let new_camp_1 = InstrCamp {
                                    label_declarations: Vec::new(),
                                    instruction: PositionedToken { 
                                        token: Token::Instruction(Instruction::Mulhl),
                                        position: instr_camp.instruction.position.clone(),
                                    },
                                    sequence: Sequence::Two (
                                        arg2.clone(),
                                        arg3.clone(),
                                    ),
                                };

                                let new_camp_2 = InstrCamp {
                                    label_declarations: Vec::new(),
                                    instruction: PositionedToken { 
                                        token: Token::Instruction(Instruction::Add),
                                        position: instr_camp.instruction.position.clone(),
                                    },
                                    sequence: Sequence::Three (
                                        arg1.clone(),
                                        zero_reg.clone(),
                                        low_reg.clone(),
                                    ),
                                };

                                ast.instr_field[instr_counter] = new_camp_1;
                                ast.instr_field[instr_counter + 1] = new_camp_2;
                            } else {
                                unreachable!()
                            }
                        }

                        
                        // ==== DIVIDE ====
                        PseudoInstruction::Div => {
                            if let Sequence::Three(arg1, arg2, arg3) = instr_camp.sequence.clone() {
                                let new_camp_1 = InstrCamp {
                                    label_declarations: Vec::new(),
                                    instruction: PositionedToken { 
                                        token: Token::Instruction(Instruction::Divhl),
                                        position: instr_camp.instruction.position.clone(),
                                    },
                                    sequence: Sequence::Two (
                                        arg2.clone(),
                                        arg3.clone(),
                                    ),
                                };

                                let new_camp_2 = InstrCamp {
                                    label_declarations: Vec::new(),
                                    instruction: PositionedToken { 
                                        token: Token::Instruction(Instruction::Add),
                                        position: instr_camp.instruction.position.clone(),
                                    },
                                    sequence: Sequence::Three (
                                        arg1.clone(),
                                        zero_reg.clone(),
                                        low_reg.clone(),
                                    ),
                                };

                                ast.instr_field[instr_counter] = new_camp_1;
                                ast.instr_field[instr_counter + 1] = new_camp_2;
                            } else {
                                unreachable!()
                            }
                        }

                        
                        // ==== MODULO ====
                        PseudoInstruction::Mod => {
                            if let Sequence::Three(arg1, arg2, arg3) = instr_camp.sequence.clone() {
                                let new_camp_1 = InstrCamp {
                                    label_declarations: Vec::new(),
                                    instruction: PositionedToken { 
                                        token: Token::Instruction(Instruction::Divhl),
                                        position: instr_camp.instruction.position.clone(),
                                    },
                                    sequence: Sequence::Two (
                                        arg2.clone(),
                                        arg3.clone(),
                                    ),
                                };

                                let new_camp_2 = InstrCamp {
                                    label_declarations: Vec::new(),
                                    instruction: PositionedToken { 
                                        token: Token::Instruction(Instruction::Add),
                                        position: instr_camp.instruction.position.clone(),
                                    },
                                    sequence: Sequence::Three (
                                        arg1.clone(),
                                        zero_reg.clone(),
                                        high_reg.clone(),
                                    ),
                                };

                                ast.instr_field[instr_counter] = new_camp_1;
                                ast.instr_field[instr_counter + 1] = new_camp_2;
                            } else {
                                unreachable!()
                            }
                        }

                        // >>>> 3 <<<<
                        // ==== SWAP ====
                        PseudoInstruction::Swap => {
                            if let Sequence::Two(arg1, arg2) = instr_camp.sequence.clone() {
                                let new_camp_1 = InstrCamp {
                                    label_declarations: Vec::new(),
                                    instruction: PositionedToken { 
                                        token: Token::Instruction(Instruction::Add),
                                        position: instr_camp.instruction.position.clone(),
                                    },
                                    sequence: Sequence::Three (
                                        aux1_reg.clone(),
                                        zero_reg.clone(),
                                        arg1.clone(),
                                    ),
                                };

                                let new_camp_2 = InstrCamp {
                                    label_declarations: Vec::new(),
                                    instruction: PositionedToken { 
                                        token: Token::Instruction(Instruction::Add),
                                        position: instr_camp.instruction.position.clone(),
                                    },
                                    sequence: Sequence::Three (
                                        arg1.clone(),
                                        zero_reg.clone(),
                                        arg2.clone(),
                                    ),
                                };

                                let new_camp_3 = InstrCamp {
                                    label_declarations: Vec::new(),
                                    instruction: PositionedToken { 
                                        token: Token::Instruction(Instruction::Add),
                                        position: instr_camp.instruction.position.clone(),
                                    },
                                    sequence: Sequence::Three (
                                        arg2.clone(),
                                        zero_reg.clone(),
                                        aux1_reg.clone(),
                                    ),
                                };

                                

                                ast.instr_field[instr_counter] = new_camp_1;
                                ast.instr_field[instr_counter + 1] = new_camp_2;
                                ast.instr_field[instr_counter + 2] = new_camp_3;
                            } else {
                                unreachable!()
                            }
                        }
                        
                        // ==== OPERATIONS IMMEDIATE ====
                        PseudoInstruction::Addi 
                        | PseudoInstruction::Subi
                        | PseudoInstruction::Andi
                        | PseudoInstruction::Ori
                        | PseudoInstruction::Xori
                        | PseudoInstruction::Shli
                        | PseudoInstruction::Shri
                        => {
                            if let Sequence::Three(arg1, arg2, arg3) = instr_camp.sequence.clone() {
                                if let Token::NumberLiteral(arg3_string) = arg3.token {
                                    let num = match u16_from_string(arg3_string) {
                                        Ok(n) => n,
                                        Err(e) => {
                                            self.exit_with_positional_error(
                                                e.as_str(),
                                                arg3.position.clone(),
                                            );
                                            unreachable!()
                                        }
                                    };
                                    let (num_low, num_high) = split_u16_to_strings(num);

                                    let new_camp_1 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken { 
                                            token: Token::Instruction(Instruction::Lli),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(num_low),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_2 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken { 
                                            token: Token::Instruction(Instruction::Lai),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(num_high),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    let operation = match instr_camp.instruction.token {
                                        Token::PseudoInstruction(PseudoInstruction::Addi) => Instruction::Add,
                                        Token::PseudoInstruction(PseudoInstruction::Subi) => Instruction::Sub,
                                        Token::PseudoInstruction(PseudoInstruction::Andi) => Instruction::And,
                                        Token::PseudoInstruction(PseudoInstruction::Ori) => Instruction::Or,
                                        Token::PseudoInstruction(PseudoInstruction::Xori) => Instruction::Xor,
                                        Token::PseudoInstruction(PseudoInstruction::Shli) => Instruction::Shl,
                                        Token::PseudoInstruction(PseudoInstruction::Shri) => Instruction::Shr,
                                        _ => unreachable!(),
                                    };

                                    let new_camp_3 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken { 
                                            token: Token::Instruction(operation),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Three(
                                            arg1.clone(),
                                            arg2.clone(),
                                            aux1_reg.clone(),
                                        ),
                                    };

                                    ast.instr_field[instr_counter] = new_camp_1;
                                    ast.instr_field[instr_counter + 1] = new_camp_2;
                                    ast.instr_field[instr_counter + 2] = new_camp_3;
                                } else {
                                    unreachable!()
                                }
                            } else {
                                unreachable!()
                            }
                        }

                        // ==== INC DEC ====
                        PseudoInstruction::Inc
                        | PseudoInstruction::Dec
                        => {
                            if let Sequence::One(arg) = instr_camp.sequence.clone() {
                                let new_camp_1 = InstrCamp {
                                    label_declarations: Vec::new(),
                                    instruction: PositionedToken {
                                        token: Token::Instruction(Instruction::Lli),
                                        position: instr_camp.instruction.position.clone(),
                                    },
                                    sequence: Sequence::Two(
                                        aux1_reg.clone(),
                                        PositionedToken {
                                            token: Token::NumberLiteral("0x01".to_string()),
                                            position: arg.position.clone(),
                                        }
                                    ),
                                };

                                let new_camp_2 = InstrCamp {
                                    label_declarations: Vec::new(),
                                    instruction: PositionedToken {
                                        token: Token::Instruction(Instruction::Lai),
                                        position: instr_camp.instruction.position.clone(),
                                    },
                                    sequence: Sequence::Two(
                                        aux1_reg.clone(),
                                        PositionedToken {
                                            token: Token::NumberLiteral("0x00".to_string()),
                                            position: arg.position.clone(),
                                        }
                                    ),
                                };

                                let op = match instr_camp.instruction.token {
                                    Token::PseudoInstruction(PseudoInstruction::Inc) => Instruction::Add,
                                    Token::PseudoInstruction(PseudoInstruction::Dec) => Instruction::Sub,
                                    _ => unreachable!(),
                                };

                                let new_camp_3 = InstrCamp {
                                    label_declarations: Vec::new(),
                                    instruction: PositionedToken {
                                        token: Token::Instruction(op),
                                        position: instr_camp.instruction.position.clone(),
                                    },
                                    sequence: Sequence::Three(
                                        arg.clone(),
                                        arg.clone(),
                                        aux1_reg.clone(),
                                    ),
                                };

                                ast.instr_field[instr_counter] = new_camp_1;
                                ast.instr_field[instr_counter + 1] = new_camp_2;
                                ast.instr_field[instr_counter + 2] = new_camp_3;
                            } else {
                                unreachable!()
                            }
                        }

                        // ==== JUMP RELATIVE ====
                        PseudoInstruction::Ja => {
                            /*
                                ja ADDRESS
                             */
                            if let Sequence::One(arg) = instr_camp.sequence.clone() {
                                if let Token::Identifier(label) = arg.token {
                                    let address = match symbol_table.get(&label) {
                                        Some(addr) => *addr,
                                        None => {
                                            self.exit_with_positional_error(
                                                "Label not found",
                                                arg.position.clone(),
                                            );
                                            unreachable!()
                                        }
                                    };
                                    let (address_low, address_high) = split_u16_to_strings(address);

                                    let new_camp_1 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken { 
                                            token: Token::Instruction(Instruction::Lli),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(address_low),
                                                position: arg.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_2 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken { 
                                            token: Token::Instruction(Instruction::Lai),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(address_high),
                                                position: arg.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_3 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken { 
                                            token: Token::Instruction(Instruction::J),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::One(
                                            aux1_reg.clone(),
                                        ),
                                    };
                                    
                                    ast.instr_field[instr_counter] = new_camp_1;
                                    ast.instr_field[instr_counter + 1] = new_camp_2;
                                    ast.instr_field[instr_counter + 2] = new_camp_3;
                                } else {
                                    unreachable!();
                                }
                            } else {
                                unreachable!();
                            }
                        }

                        // >>>> 6 <<<<

                        // ====STORE BYTE ====
                       PseudoInstruction::Sb
                        => {
                            if let Sequence::Three(arg1, arg2, arg3) = instr_camp.sequence.clone() {
                                // sb $rd, $rs[offset]
                                if let Token::NumberLiteral(offset_str) = arg3.token.clone() {
                                    let offset = match u16_from_string(offset_str) {
                                        Ok(n) => n,
                                        Err(e) => {
                                            self.exit_with_positional_error(
                                                e.as_str(),
                                                arg3.position.clone(),
                                            );
                                            unreachable!()
                                        }
                                    };

                                    let (offset_low, offset_high) = split_u16_to_strings(offset);
                                    let new_camp_1 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lli),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(offset_low),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };
                                    let new_camp_2 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lai),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(offset_high),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    // add $aux1, $aux1, $rs
                                    let new_camp_3 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Add),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Three(
                                            aux1_reg.clone(),
                                            aux1_reg.clone(),
                                            arg2.clone(),
                                        ),
                                    };

                                    // slb $rd, $aux1

                                    let new_camp_4 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Slb),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            arg1.clone(),
                                            aux1_reg.clone(),
                                        ),
                                    };

                                    ast.instr_field[instr_counter] = new_camp_1;
                                    ast.instr_field[instr_counter + 1] = new_camp_2;
                                    ast.instr_field[instr_counter + 2] = new_camp_3;
                                    ast.instr_field[instr_counter + 3] = new_camp_4;

                                } else {
                                    unreachable!()
                                }
                            } else {
                                unreachable!()
                            }
                        }
                        
                        // ==== MULTIPLY IMMEDIATE ====
                        PseudoInstruction::Muli => {
                            if let Sequence::Three(arg1, arg2, arg3) = instr_camp.sequence.clone() {
                                if let Token::NumberLiteral(imm_str) = arg3.token {
                                    let num = match u16_from_string(imm_str) {
                                        Ok(n) => n,
                                        Err(e) => {
                                            self.exit_with_positional_error(
                                                e.as_str(),
                                                arg3.position.clone(),
                                            );
                                            unreachable!()
                                        }
                                    };
                                    let (num_low, num_high) = split_u16_to_strings(num);

                                    let new_camp_1 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lli),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(num_low),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_2 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lai),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(num_high),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_3 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Mulhl),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            arg2.clone(),
                                            aux1_reg.clone(),
                                        ),
                                    };

                                    let new_camp_4 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Add),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Three(
                                            arg1.clone(),
                                            zero_reg.clone(),
                                            low_reg.clone(),
                                        ),
                                    };

                                    ast.instr_field[instr_counter] = new_camp_1;
                                    ast.instr_field[instr_counter + 1] = new_camp_2;
                                    ast.instr_field[instr_counter + 2] = new_camp_3;
                                    ast.instr_field[instr_counter + 3] = new_camp_4;
                                } else {
                                    unreachable!()
                                }
                            } else {
                                unreachable!()
                            }
                        }

                        // ==== DIVIDE IMMEDIATE ====
                        PseudoInstruction::Divi => {
                            if let Sequence::Three(arg1, arg2, arg3) = instr_camp.sequence.clone() {
                                if let Token::NumberLiteral(imm_str) = arg3.token {
                                    let num = match u16_from_string(imm_str) {
                                        Ok(n) => n,
                                        Err(e) => {
                                            self.exit_with_positional_error(
                                                e.as_str(),
                                                arg3.position.clone(),
                                            );
                                            unreachable!()
                                        }
                                    };
                                    let (num_low, num_high) = split_u16_to_strings(num);

                                    let new_camp_1 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lli),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(num_low),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_2 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lai),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(num_high),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_3 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Divhl),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            arg2.clone(),
                                            aux1_reg.clone(),
                                        ),
                                    };

                                    let new_camp_4 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Add),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Three(
                                            arg1.clone(),
                                            zero_reg.clone(),
                                            low_reg.clone(),
                                        ),
                                    };

                                    ast.instr_field[instr_counter] = new_camp_1;
                                    ast.instr_field[instr_counter + 1] = new_camp_2;
                                    ast.instr_field[instr_counter + 2] = new_camp_3;
                                    ast.instr_field[instr_counter + 3] = new_camp_4;
                                } else {
                                    unreachable!()
                                }
                            } else {
                                unreachable!()
                            }
                        }

                        // ==== MODULO IMMEDIATE ====
                        PseudoInstruction::Modi => {
                            if let Sequence::Three(arg1, arg2, arg3) = instr_camp.sequence.clone() {
                                if let Token::NumberLiteral(imm_str) = arg3.token {
                                    let num = match u16_from_string(imm_str) {
                                        Ok(n) => n,
                                        Err(e) => {
                                            self.exit_with_positional_error(
                                                e.as_str(),
                                                arg3.position.clone(),
                                            );
                                            unreachable!()
                                        }
                                    };
                                    let (num_low, num_high) = split_u16_to_strings(num);

                                    let new_camp_1 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lli),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(num_low),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_2 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lai),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(num_high),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_3 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Divhl),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            arg2.clone(),
                                            aux1_reg.clone(),
                                        ),
                                    };

                                    let new_camp_4 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Add),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Three(
                                            arg1.clone(),
                                            zero_reg.clone(),
                                            high_reg.clone(),
                                        ),
                                    };

                                    ast.instr_field[instr_counter] = new_camp_1;
                                    ast.instr_field[instr_counter + 1] = new_camp_2;
                                    ast.instr_field[instr_counter + 2] = new_camp_3;
                                    ast.instr_field[instr_counter + 3] = new_camp_4;
                                } else {
                                    unreachable!()
                                }
                            } else {
                                unreachable!()
                            }
                        }

                        PseudoInstruction::Lb // -- 7
                        => {
                            if let Sequence::Three(arg1, arg2, arg3) = instr_camp.sequence.clone() {
                                // lb $rd, $rs[offset]
                                if let Token::NumberLiteral(offset_str) = arg3.token {
                                    let offset = match u16_from_string(offset_str) {
                                        Ok(n) => n,
                                        Err(e) => {
                                            self.exit_with_positional_error(
                                                e.as_str(),
                                                arg3.position.clone(),
                                            );
                                            unreachable!()
                                        }
                                    };

                                    let (offset_low, offset_high) = split_u16_to_strings(offset);
                                    let new_camp_1 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lli),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(offset_low),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };
                                    let new_camp_2 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lai),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(offset_high),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    // add $aux1, $aux1, $rs
                                    let new_camp_3 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Add),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Three(
                                            aux1_reg.clone(),
                                            aux1_reg.clone(),
                                            arg2.clone(),
                                        ),
                                    };

                                    // llb $rd, $aux1
                                    let new_camp_4 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Llb),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            arg1.clone(),
                                            aux1_reg.clone(),
                                        ),
                                    };

                                    // xlb $rd, $rd

                                    let new_camp_5 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Xlb),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            arg1.clone(),
                                            arg1.clone(),
                                        ),
                                    };

                                    ast.instr_field[instr_counter] = new_camp_1;
                                    ast.instr_field[instr_counter + 1] = new_camp_2;
                                    ast.instr_field[instr_counter + 2] = new_camp_3;
                                    ast.instr_field[instr_counter + 3] = new_camp_4;
                                    ast.instr_field[instr_counter + 4] = new_camp_5;
                                } else {
                                    unreachable!()
                                }
                            } else {
                                unreachable!()
                            }
                        }
                        // ==== BRANCH INSTRUCTIONS (3 instructions)====
                        PseudoInstruction::Beqa
                        | PseudoInstruction::Bneqa
                        | PseudoInstruction::Blta
                        | PseudoInstruction::Bgta
                        | PseudoInstruction::Bltua
                        | PseudoInstruction::Bgtua
                        => {
                            if let Sequence::Three(arg1, arg2, arg3) = instr_camp.sequence.clone() {
                                if let Token::Identifier(label) = arg3.token {
                                    let address = match symbol_table.get(&label) {
                                        Some(addr) => *addr,
                                        None => {
                                            self.exit_with_positional_error(
                                                "Label not found",
                                                arg3.position.clone(),
                                            );
                                            unreachable!()
                                        }
                                    };
                                    
                                    let relative_target_address = address.wrapping_sub(instr_counter as u16).wrapping_sub(2);
                                    
                                    let (relative_target_address_low, relative_target_address_high) = split_u16_to_strings(relative_target_address);

                                    let new_camp_1 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lli),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(relative_target_address_low),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_2 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lai),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(relative_target_address_high),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    

                                    let branch_instr = match instr_camp.instruction.token {
                                        Token::PseudoInstruction(PseudoInstruction::Beqa) => Instruction::Beqr,
                                        Token::PseudoInstruction(PseudoInstruction::Bneqa) => Instruction::Bneqr,
                                        Token::PseudoInstruction(PseudoInstruction::Blta) => Instruction::Bltr,
                                        Token::PseudoInstruction(PseudoInstruction::Bgta) => Instruction::Bgtr,
                                        Token::PseudoInstruction(PseudoInstruction::Bltua) => Instruction::Bltur,
                                        Token::PseudoInstruction(PseudoInstruction::Bgtua) => Instruction::Bgtur,
                                        _ => unreachable!(),
                                    };

                                    let new_camp_3 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(branch_instr),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Three(
                                            arg1.clone(),
                                            arg2.clone(),
                                            aux1_reg.clone(),
                                        ),
                                    };

                                    
                                    ast.instr_field[instr_counter] = new_camp_1;
                                    ast.instr_field[instr_counter + 1] = new_camp_2;
                                    ast.instr_field[instr_counter + 2] = new_camp_3;
                                } else {
                                    unreachable!()
                                }
                            } else {
                                unreachable!()
                            }
                        }

                       // >>>> 10 <<<<
                       // ==== LOAD WORD and STORE WORD ====
                        PseudoInstruction::Lw
                        | PseudoInstruction::Sw
                        => {
                            if let Sequence::Three(arg1, arg2, arg3) = instr_camp.sequence.clone() {
                                if let Token::NumberLiteral(offset_str) = arg3.token {
                                    // lw $rd, $rs[offset]
                                    let offset = match u16_from_string(offset_str) {
                                        Ok(n) => n,
                                        Err(e) => {
                                            self.exit_with_positional_error(
                                                e.as_str(),
                                                arg3.position.clone(),
                                            );
                                            unreachable!()
                                        }
                                    };

                                    let (offset_low, offset_high) = split_u16_to_strings(offset);

                                    // la $aux1, offset

                                    let new_camp_1 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lli),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(offset_low),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_2 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lai),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux1_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral(offset_high),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    // add $aux1, $aux1, $rs

                                    let new_camp_3 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Add),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Three(
                                            aux1_reg.clone(),
                                            aux1_reg.clone(),
                                            arg2.clone(),
                                        ),
                                    };

                                    // lab $rd, $aux1 or sab $rd, $aux1

                                    let new_camp_4 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(
                                                match instr_camp.instruction.token {
                                                    Token::PseudoInstruction(PseudoInstruction::Lw) => Instruction::Lab,
                                                    Token::PseudoInstruction(PseudoInstruction::Sw) => Instruction::Sab,
                                                    _ => unreachable!(),
                                                }
                                            ),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            arg1.clone(),
                                            aux1_reg.clone(),
                                        ),
                                    };

                                    // la $aux2, 0x0001
                                    let new_camp_5 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lli),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux2_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral("0x01".to_string()),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    let new_camp_6 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Lai),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            aux2_reg.clone(),
                                            PositionedToken {
                                                token: Token::NumberLiteral("0x00".to_string()),
                                                position: arg3.position.clone(),
                                            }
                                        ),
                                    };

                                    // add $aux1, $aux1, $aux2
                                    let new_camp_7 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(Instruction::Add),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Three(
                                            aux1_reg.clone(),
                                            aux1_reg.clone(),
                                            aux2_reg.clone(),
                                        ),
                                    };

                                    // llb $rd, $aux1 or slb $rd, $aux1
                                    let new_camp_8 = InstrCamp {
                                        label_declarations: Vec::new(),
                                        instruction: PositionedToken {
                                            token: Token::Instruction(
                                                match instr_camp.instruction.token {
                                                    Token::PseudoInstruction(PseudoInstruction::Lw) => Instruction::Llb,
                                                    Token::PseudoInstruction(PseudoInstruction::Sw) => Instruction::Slb,
                                                    _ => unreachable!(),
                                                }
                                            ),
                                            position: instr_camp.instruction.position.clone(),
                                        },
                                        sequence: Sequence::Two(
                                            arg1.clone(), 
                                            aux1_reg.clone()
                                        ),
                                    };

                                    ast.instr_field[instr_counter] = new_camp_1;
                                    ast.instr_field[instr_counter + 1] = new_camp_2;
                                    ast.instr_field[instr_counter + 2] = new_camp_3;
                                    ast.instr_field[instr_counter + 3] = new_camp_4;
                                    ast.instr_field[instr_counter + 4] = new_camp_5;
                                    ast.instr_field[instr_counter + 5] = new_camp_6;
                                    ast.instr_field[instr_counter + 6] = new_camp_7;
                                    ast.instr_field[instr_counter + 7] = new_camp_8;

                                } else {
                                    unreachable!()
                                }
                            } else {
                                unreachable!()
                            }
                        }
                    }
                    instr_counter += 1;
                    continue;
                }
                _ => {
                    instr_counter += 1;
                    continue;
                }
            }
        }
    }
}