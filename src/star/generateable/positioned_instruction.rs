use crate::star::utils::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PositionedInstruction {
    pub format: u16,
    pub position: Position,
}