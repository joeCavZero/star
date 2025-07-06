use colored::Colorize;
use supports_color::Stream;

use crate::star::utils::Stringable;

const INTERPRETER_NAME: &str = "STAR";

pub fn interpreter() -> String {
    let text = format!("[{}]", INTERPRETER_NAME);
    if let Some(color_level) = supports_color::on(Stream::Stdout) {
        if color_level.has_16m || color_level.has_256 {
            text
                .bold()
                .yellow()
                .to_string()
        } else {
            text
        }
    } else {
        text
    }
}

pub fn error() -> String {
    let text = "[error]".to_string();
    if let Some(color_level) = supports_color::on(Stream::Stdout) {
        if color_level.has_16m || color_level.has_256 {
            text
                .bold()
                .bright_red()
                .to_string()
        } else {
            text
        }
    } else {
        text
    }
}

pub fn position(file: String, line: u32, column: Option<u32>) -> String {
    
    let text = format!("[file: {}, line: {}, column: {}]", file.beautiful_path(), line, column.unwrap_or(0));
    if let Some(color_level) = supports_color::on(Stream::Stdout) {
        if color_level.has_16m || color_level.has_256 {
            text
                .bold()
                .magenta()
                .to_string()
        } else {
            text
        }
    } else {
        text
    }
}