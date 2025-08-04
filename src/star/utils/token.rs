use crate::star::utils::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Processor(Processor),
    GeneralRegister(GeneralRegister),
    Instruction(Instruction),
    PseudoInstruction(PseudoInstruction),
    LabelDeclaration(String),
    Identifier(String),
    MacroArgIdentifier(String),
    Directive(Directive),
    NumberLiteral(String),
    StringLiteral(String),
    Comma,
    LeftSquareBracket,
    RightSquareBracket,
    LeftParenthesis,
    RightParenthesis,
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
            
            "xlb" => Ok(Token::Instruction(Instruction::Xlb)),
            "lab" => Ok(Token::Instruction(Instruction::Lab)),
            "llb" => Ok(Token::Instruction(Instruction::Llb)),
            "sab" => Ok(Token::Instruction(Instruction::Sab)),
            "slb" => Ok(Token::Instruction(Instruction::Slb)),
            "mulhl" => Ok(Token::Instruction(Instruction::Mulhl)),
            "divhl" => Ok(Token::Instruction(Instruction::Divhl)),
            "muluhl" => Ok(Token::Instruction(Instruction::Muluhl)),
            "divuhl" => Ok(Token::Instruction(Instruction::Divuhl)),
            "not" => Ok(Token::Instruction(Instruction::Not)),

            "j" => Ok(Token::Instruction(Instruction::J)),

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
            "ja" => Ok(Token::PseudoInstruction(PseudoInstruction::Ja)),
            "jr" => Ok(Token::PseudoInstruction(PseudoInstruction::Jr)),

            "ret" => Ok(Token::PseudoInstruction(PseudoInstruction::Ret)),
            // ==== MISC ====
            "," => Ok(Token::Comma),
            "[" => Ok(Token::LeftSquareBracket),
            "]" => Ok(Token::RightSquareBracket),
            "(" => Ok(Token::LeftParenthesis),
            ")" => Ok(Token::RightParenthesis),
            "\\" => Ok(Token::Backslash),

            // ==== PROCESSORS ====
            _ if tkn_string.starts_with("@") => {
                match tkn_string.as_str() {
                    "@include" => Ok(Token::Processor(Processor::Include)),
                    "@define" => Ok(Token::Processor(Processor::Define)),
                    "@once" => Ok(Token::Processor(Processor::Once)),
                    _ => Err("Invalid processor".to_string()),
                }
            }

            // ==== REGISTERS ====
            _ if tkn_string.starts_with("$") => {
                match tkn_string.as_str() {
                    "$zero" | "$0" => Ok(Token::GeneralRegister(GeneralRegister::Zero)),
                    "$a" | "$1" => Ok(Token::GeneralRegister(GeneralRegister::A)),
                    "$b" | "$2" => Ok(Token::GeneralRegister(GeneralRegister::B)),
                    "$c" | "$3" => Ok(Token::GeneralRegister(GeneralRegister::C)),
                    "$d" | "$4" => Ok(Token::GeneralRegister(GeneralRegister::D)),
                    "$e" | "$5" => Ok(Token::GeneralRegister(GeneralRegister::E)),
                    "$f" | "$6" => Ok(Token::GeneralRegister(GeneralRegister::F)),
                    "$g" | "$7" => Ok(Token::GeneralRegister(GeneralRegister::G)),
                    "$aux1" | "$8" => Ok(Token::GeneralRegister(GeneralRegister::Aux1)),
                    "$aux2" | "$9" => Ok(Token::GeneralRegister(GeneralRegister::Aux2)),
                    "$aux3" | "$10" => Ok(Token::GeneralRegister(GeneralRegister::Aux3)),
                    "$carry" | "$11" => Ok(Token::GeneralRegister(GeneralRegister::Carry)),
                    "$low" | "$12" => Ok(Token::GeneralRegister(GeneralRegister::Low)),
                    "$high" | "$13" => Ok(Token::GeneralRegister(GeneralRegister::High)),
                    "$ra" | "$14" => Ok(Token::GeneralRegister(GeneralRegister::ReturnAddress)),
                    "$sp" | "$15" => Ok(Token::GeneralRegister(GeneralRegister::StackPointer)),
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
                    ".checkpoint" => Ok(Token::Directive(Directive::Checkpoint)),
                    _ => Err("Invalid directive".to_string()),
                }
            }

            // ==== MACRO ARGUMENT ====
            _ if tkn_string.starts_with("%") => {
                let arg_name = tkn_string.trim_start_matches('%').to_string();
                Ok(Token::MacroArgIdentifier(arg_name))
            }

            // ==== LABEL DECLARATIONS ====
            _ if tkn_string.ends_with(":") => {
                let label = tkn_string.trim_end_matches(':').to_string();
                Ok(Token::LabelDeclaration(label))
            }

            // ==== STRING LITERALS ====
            _ if tkn_string.starts_with("\"") && tkn_string.ends_with("\"") => {
                let string_literal = tkn_string[1..tkn_string.len()-1].to_string();
                Ok(Token::StringLiteral(string_literal.processed_string()))
            }

            // ==== NUMBERS ====
            _ if tkn_string.to_lowercase().starts_with("0x") 
            || tkn_string.to_lowercase().starts_with("0b") 
            || (
                !tkn_string.starts_with("_") &&
                tkn_string.replace("_","").parse::<i64>().is_ok() 
            )
            => {
                Ok(Token::NumberLiteral(tkn_string))
            }

            // ==== IDENTIFIERS ====
            _ => Ok(Token::Identifier(tkn_string)),
        }
    }

}