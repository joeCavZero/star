use crate::star::utils::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PositionedToken {
    pub token: Token,
    pub position: Position,
}