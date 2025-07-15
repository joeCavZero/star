#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Processor {
    Include,
    Define,
    Once,
}