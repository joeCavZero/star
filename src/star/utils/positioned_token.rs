use crate::star::utils::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PositionedToken {
    pub token: Token,
    pub position: Position,
}