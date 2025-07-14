#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Directive {
    Data,
    Instr,
    Byte,
    Word,
    Space,
    String,
    Stringz,
    Checkpoint,
}