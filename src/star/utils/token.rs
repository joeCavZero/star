use crate::star::utils::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Processor(Processor),
    Register(Register),
    Instruction(Instruction),
    PseudoInstruction(PseudoInstruction),
    LabelDeclaration(String),
    Identifier(String),
    Directive(Directive),
    NumberLiteral(String),
    StringLiteral(String),
    Comma,
    LeftSquareBracket,
    RightSquareBracket,
    Backslash,
}

impl Token {
    pub fn from_string(tkn_string: String) -> Result<Self, String> {
        match tkn_string.as_str() {
            "lai" => Ok(Token::Instruction(Instruction::Lai)),
            "lli" => Ok(Token::Instruction(Instruction::Lli)),
            "add" => Ok(Token::Instruction(Instruction::Add)),
            "sub" => Ok(Token::Instruction(Instruction::Sub)),
            "and" => Ok(Token::Instruction(Instruction::And)),
            "or" => Ok(Token::Instruction(Instruction::Or)),
            "xor" => Ok(Token::Instruction(Instruction::Xor)),
            "shl" => Ok(Token::Instruction(Instruction::Shl)),
            "shr" => Ok(Token::Instruction(Instruction::Shr)),
            
            "beqr" => Ok(Token::Instruction(Instruction::Beqr)),
            "bneqr" => Ok(Token::Instruction(Instruction::Bneqr)),
            "bgtr" => Ok(Token::Instruction(Instruction::Bgtr)),
            "bltr" => Ok(Token::Instruction(Instruction::Bltr)),
            "bgtur" => Ok(Token::Instruction(Instruction::Bgtur)),
            "bltur" => Ok(Token::Instruction(Instruction::Bltur)),
            
            "xb" => Ok(Token::Instruction(Instruction::Xb)),
            "lab" => Ok(Token::Instruction(Instruction::Lab)),
            "llb" => Ok(Token::Instruction(Instruction::Llb)),
            "sab" => Ok(Token::Instruction(Instruction::Sab)),
            "slb" => Ok(Token::Instruction(Instruction::Slb)),
            "mulhl" => Ok(Token::Instruction(Instruction::Mulhl)),
            "divhl" => Ok(Token::Instruction(Instruction::Divhl)),
            "muluhl" => Ok(Token::Instruction(Instruction::Muluhl)),
            "divuhl" => Ok(Token::Instruction(Instruction::Divuhl)),
            "not" => Ok(Token::Instruction(Instruction::Not)),
            "jar" => Ok(Token::Instruction(Instruction::Jar)),

            "mcall" => Ok(Token::Instruction(Instruction::Mcall)),
            

            // ==== PSEUDO INSTRUCTIONS ====
            "nope" => Ok(Token::PseudoInstruction(PseudoInstruction::Nope)),

            "move" => Ok(Token::PseudoInstruction(PseudoInstruction::Move)),
            "swap" => Ok(Token::PseudoInstruction(PseudoInstruction::Swap)),
            "la" => Ok(Token::PseudoInstruction(PseudoInstruction::La)),
            
            "lb" => Ok(Token::PseudoInstruction(PseudoInstruction::Lb)),
            "lw" => Ok(Token::PseudoInstruction(PseudoInstruction::Lw)),
            
            "li" => Ok(Token::PseudoInstruction(PseudoInstruction::Li)),
            
            "sb" => Ok(Token::PseudoInstruction(PseudoInstruction::Sb)),
            "sw" => Ok(Token::PseudoInstruction(PseudoInstruction::Sw)),
            
            
            "addi" => Ok(Token::PseudoInstruction(PseudoInstruction::Addi)),
            "subi" => Ok(Token::PseudoInstruction(PseudoInstruction::Subi)),
            "andi" => Ok(Token::PseudoInstruction(PseudoInstruction::Andi)),
            "ori" => Ok(Token::PseudoInstruction(PseudoInstruction::Ori)),
            "xori" => Ok(Token::PseudoInstruction(PseudoInstruction::Xori)),
            "shli" => Ok(Token::PseudoInstruction(PseudoInstruction::Shli)),
            "shri" => Ok(Token::PseudoInstruction(PseudoInstruction::Shri)),
            
            "neg" => Ok(Token::PseudoInstruction(PseudoInstruction::Neg)),
            "inc" => Ok(Token::PseudoInstruction(PseudoInstruction::Inc)),
            "dec" => Ok(Token::PseudoInstruction(PseudoInstruction::Dec)),
            
            "mul" => Ok(Token::PseudoInstruction(PseudoInstruction::Mul)),
            "div" => Ok(Token::PseudoInstruction(PseudoInstruction::Div)),
            "mod" => Ok(Token::PseudoInstruction(PseudoInstruction::Mod)),

            "muli" => Ok(Token::PseudoInstruction(PseudoInstruction::Muli)),
            "divi" => Ok(Token::PseudoInstruction(PseudoInstruction::Divi)),
            "modi" => Ok(Token::PseudoInstruction(PseudoInstruction::Modi)),
            
            "beqa" => Ok(Token::PseudoInstruction(PseudoInstruction::Beqa)),
            "bneqa" => Ok(Token::PseudoInstruction(PseudoInstruction::Bneqa)),
            "bgta" => Ok(Token::PseudoInstruction(PseudoInstruction::Bgta)),
            "blta" => Ok(Token::PseudoInstruction(PseudoInstruction::Blta)),
            "bgtua" => Ok(Token::PseudoInstruction(PseudoInstruction::Bgtua)),
            "bltua" => Ok(Token::PseudoInstruction(PseudoInstruction::Bltua)),
            "Ja" => Ok(Token::PseudoInstruction(PseudoInstruction::Ja)),
            "jr" => Ok(Token::PseudoInstruction(PseudoInstruction::Jr)),

            "ret" => Ok(Token::PseudoInstruction(PseudoInstruction::Ret)),
            // ==== MISC ====
            "," => Ok(Token::Comma),
            "[" => Ok(Token::LeftSquareBracket),
            "]" => Ok(Token::RightSquareBracket),
            "\\" => Ok(Token::Backslash),

            // ==== PROCESSORS ====
            _ if tkn_string.starts_with("@") => {
                match tkn_string.as_str() {
                    "@include" => Ok(Token::Processor(Processor::Include)),
                    "@define" => Ok(Token::Processor(Processor::Define)),
                    _ => Err("Invalid processor".to_string()),
                }
            }

            // ==== REGISTERS ====
            _ if tkn_string.starts_with("$") => {
                match tkn_string.as_str() {
                    "$zero" | "$0" => Ok(Token::Register(Register::Zero)),
                    "$a" | "$1" => Ok(Token::Register(Register::A)),
                    "$b" | "$2" => Ok(Token::Register(Register::B)),
                    "$c" | "$3" => Ok(Token::Register(Register::C)),
                    "$d" | "$4" => Ok(Token::Register(Register::D)),
                    "$e" | "$5" => Ok(Token::Register(Register::E)),
                    "$f" | "$6" => Ok(Token::Register(Register::F)),
                    "$g" | "$7" => Ok(Token::Register(Register::G)),
                    "$h" | "$8" => Ok(Token::Register(Register::H)),
                    "$aux1" | "$9" => Ok(Token::Register(Register::Aux1)),
                    "$aux2" | "$10" => Ok(Token::Register(Register::Aux2)),
                    "$aux3" | "$11" => Ok(Token::Register(Register::Aux3)),
                    "$carry" | "$12" => Ok(Token::Register(Register::Carry)),
                    "$high" | "$13" => Ok(Token::Register(Register::High)),
                    "$low" | "$14" => Ok(Token::Register(Register::Low)),
                    "$ra" | "$15" => Ok(Token::Register(Register::ReturnAddress)),
                    _ => Err("Invalid register".to_string()),
                }
            }

            // ==== DIRECTIVES ====
            _ if tkn_string.starts_with(".") => {
                match tkn_string.as_str() {
                    ".data" => Ok(Token::Directive(Directive::Data)),
                    ".instr" => Ok(Token::Directive(Directive::Instr)),
                    ".byte" => Ok(Token::Directive(Directive::Byte)),
                    ".word" => Ok(Token::Directive(Directive::Word)),
                    ".space" => Ok(Token::Directive(Directive::Space)),
                    ".string" => Ok(Token::Directive(Directive::String)),
                    ".stringz" => Ok(Token::Directive(Directive::Stringz)),
                    _ => Err("Invalid directive".to_string()),
                }
            }

            // ==== LABEL DECLARATIONS ====
            _ if tkn_string.ends_with(":") => {
                let label = tkn_string.trim_end_matches(':').to_string();
                Ok(Token::LabelDeclaration(label))
            }

            // ==== STRING LITERALS ====
            _ if tkn_string.starts_with("\"") && tkn_string.ends_with("\"") => {
                let string_literal = tkn_string.trim_matches('"').to_string();
                Ok(Token::StringLiteral(string_literal.processed_string()))
            }

            // ==== NUMBERS ====
            _ if tkn_string.to_lowercase().starts_with("0x") || tkn_string.to_lowercase().starts_with("0b") || tkn_string.parse::<i32>().is_ok() => {
                Ok(Token::NumberLiteral(tkn_string))
            }

            // ==== IDENTIFIERS ====
            _ => Ok(Token::Identifier(tkn_string)),
        }
    }
}