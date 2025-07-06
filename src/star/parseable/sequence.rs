use crate::star::utils::*;

#[derive(Debug, Clone)]
pub enum Sequence {
    Zero,
    One(PositionedToken),
    Two(PositionedToken, PositionedToken),
    Three(PositionedToken, PositionedToken, PositionedToken),
}