#[derive(Debug, Clone, PartialEq)]
pub enum Directive {
    Data,
    Instr,
    Byte,
    Word,
    Space,
    String,
    Stringz,
}