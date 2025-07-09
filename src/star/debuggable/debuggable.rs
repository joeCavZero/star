use crate::star::core::*;
use crate::star::utils::*;

use crate::debugger;


pub trait Debugable {
    fn exit_with_positional_error(&self, error: &str, position: Position);
    fn exit_with_optional_positional_error(&self, error: &str, position: Option<Position>);
}

impl Debugable for Star {
    fn exit_with_positional_error(&self, error: &str, position: Position) {
        println!(
            "\n{} {} {} {}",
            debugger::interpreter(),
            debugger::error(),
            error,
            debugger::position(
                self.get_file_name(position.file),
                position.line,
                position.column
            ),
        );
        std::process::exit(0);
    }

    fn exit_with_optional_positional_error(&self, error: &str, position: Option<Position>) {
        if let Some(pos) = position {
            self.exit_with_positional_error(error, pos);
        } else {
            debugger::exit_with_error(error);
        }
    }
}