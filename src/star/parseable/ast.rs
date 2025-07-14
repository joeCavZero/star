use crate::star::utils::*;
use super::sequence::*;

#[derive(Debug, Clone)]
pub struct InstrCamp {
    pub label_declarations: Vec<PositionedToken>,
    pub instruction: PositionedToken,
    pub sequence: Sequence,
}

#[derive(Debug, Clone)]
pub struct DataCamp {
    pub label_declarations: Vec<PositionedToken>,
    pub directive: PositionedToken,
    pub arg: DataCampArg,
}

#[derive(Debug, Clone)]
pub enum DataCampArg {
    Empty,
    Unique(PositionedToken),
    Multiple(Vec<PositionedToken>),
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub data_field: Vec<DataCamp>,
    pub instr_field: Vec<InstrCamp>
}

