use crate::star::core::*;
use crate::star::utils::*;

use super::debug;


pub trait Debugable {
    fn exit_with_error(&self, error: &str);
    fn exit_with_positional_error(&self, error: &str, position: Position);
    fn exit_with_optional_positional_error(&self, error: &str, position: Option<Position>);
}

impl Debugable for Star {
    fn exit_with_positional_error(&self, error: &str, position: Position) {
        println!(
            "\n{} {} {} {}",
            debug::interpreter(),
            debug::error(),
            error,
            debug::position(
                self.get_file_name(position.file),
                position.line,
                position.column
            ),
        );
        std::process::exit(0);
    }

    fn exit_with_error(&self, error: &str) {
        println!(
            "\n{} {} {}",
            debug::interpreter(),
            debug::error(),
            error
        );
        std::process::exit(0);
    }

    fn exit_with_optional_positional_error(&self, error: &str, position: Option<Position>) {
        if let Some(pos) = position {
            self.exit_with_positional_error(error, pos);
        } else {
            self.exit_with_error(error);
        }
    }
}