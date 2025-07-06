use crate::star::core::*;
use crate::star::parseable::*;
use crate::star::symbolable::*;
use crate::star::utils::*;

/*
    Theres is a difference between resolving non-adressed and
    addressed instructions. Non-adressed instructions does not
    require a symbol table to be resolved, while addressed
    instructions require a symbol table to be resolved.
    e.g.:
        - beqa needs the symnol table to use his address
        numbers
        - like...
        ```
        0 :: beqa $r1, $r2, <LABEL>
        1 :: nope
        2 :: nope
        3 :: nope
        4 :: nope
        5 :: LABEL: instr
        ```
            |
            V
        ```
        0 :: li $aux1, <LABEL>
        1 :: lr $aux1, $aux1
        2 :: li $aux2, 0x0004
        3 :: sub $aux1, $aux1, $aux2
        4 :: beqr $r1, $r2, $aux1
        5 :: LABEL: instr
        ```
    - the first layer will add nopes after pseudo instructions to
    resolve the instruction PC differences.

    - After it, we will resolve the symbol table.

    - After the symbol table is resolved, we will resolve
    the second layer, which will resolve the addresses of the
    instructions.
        
*/
pub trait Resolveable {
    fn resolve(&self, ast: &mut Ast);

    fn resolve_first_layer(&self, ast: &mut Ast);
}

impl Resolveable for Star {
    fn resolve(&self, ast: &mut Ast) {
        self.resolve_first_layer(ast);
        let symbol_table = self.get_symbol_table(ast);
        println!("\n\n\n{:#?}", symbol_table);
    }
    fn resolve_first_layer(&self, ast: &mut Ast) {
        let mut instr_field_len = ast.instr_field.len();
        let mut instr_counter: usize = 0;
        while instr_counter < instr_field_len {
            let instr_camp = match ast.instr_field.get(instr_counter) {
                Some(camp) => camp,
                None => break,
            };
            let nope_camp = InstrCamp {
                label_declarations: Vec::new(),
                instruction: PositionedToken {
                    token: Token::Instruction(Instruction::Nope),
                    position: instr_camp.instruction.position,
                },
                sequence: Sequence::Zero,
            };

            
            match instr_camp.instruction.token.clone() {
                Token::PseudoInstruction(pseudo_instruction) => {
                    match pseudo_instruction {
                        // ==== +0 ====
                        PseudoInstruction::Move
                        | PseudoInstruction::Neg
                        => {}
                        // ==== +1 ====
                        PseudoInstruction::Li
                        | PseudoInstruction::La
                        | PseudoInstruction::Mul
                        | PseudoInstruction::Div
                        | PseudoInstruction::Mod
                        | PseudoInstruction::Mulu
                        | PseudoInstruction::Divu
                        | PseudoInstruction::Modu
                        => {
                            ast.instr_field.insert(instr_counter + 1, nope_camp.clone());

                            instr_counter += 1;
                            instr_field_len = ast.instr_field.len();
                            continue;
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

                        | PseudoInstruction::Inc
                        | PseudoInstruction::Dec
                        => {
                            for _ in 0..2 {
                                ast.instr_field.insert(instr_counter + 1, nope_camp.clone());
                            }

                            instr_counter += 1;
                            instr_field_len = ast.instr_field.len();
                            continue;
                        }

                        // ==== +3 ====
                        PseudoInstruction::Lb
                        | PseudoInstruction::Sb
                        | PseudoInstruction::Muli
                        | PseudoInstruction::Divi
                        | PseudoInstruction::Modi
                        | PseudoInstruction::Mului
                        | PseudoInstruction::Divui
                        | PseudoInstruction::Modui
                        => {
                            for _ in 0..3 {
                                ast.instr_field.insert(instr_counter + 1, nope_camp.clone());
                            }

                            instr_counter += 1;
                            instr_field_len = ast.instr_field.len();
                            continue;
                        }

                        // ==== +6 ====
                        PseudoInstruction::Beqa
                        | PseudoInstruction::Bneqa
                        | PseudoInstruction::Blta
                        | PseudoInstruction::Bgta
                        | PseudoInstruction::Bltua
                        | PseudoInstruction::Bgtua
                        | PseudoInstruction::Ba
                        => {
                
                            for _ in 0..6 {
                                ast.instr_field.insert(instr_counter + 1, nope_camp.clone());
                            }
                            
                            instr_counter += 1;
                            instr_field_len = ast.instr_field.len();
                            continue;
                        }

                        // ==== +7 ====
                        PseudoInstruction::Lw
                        | PseudoInstruction::Sw
                        => {
                            for _ in 0..7 {
                                ast.instr_field.insert(instr_counter + 1, nope_camp.clone());
                            }

                            instr_counter += 1;
                            instr_field_len = ast.instr_field.len();
                            continue;
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
}