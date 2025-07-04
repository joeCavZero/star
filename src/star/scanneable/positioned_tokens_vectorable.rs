use crate::star::utils::*;

pub trait PositionedTokensVectorable {
    fn push_positioned_token(
        &mut self,
        token_string: String,
        file_id: u32,
        line: u32,
        column: Option<u32>,
    ) -> Result<(), String>;
}

impl PositionedTokensVectorable for Vec<PositionedToken> {
    fn push_positioned_token(
        &mut self,
        token_string: String,
        file_id: u32,
        line: u32,
        column: Option<u32>,
    ) -> Result<(), String> {
        let tkn = Token::from_string(token_string);
        match tkn {
            Ok(token) => {
                self.push(PositionedToken {
                    token,
                    position: Position::new(
                        file_id,
                        line,
                        column,
                    )
                });
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}