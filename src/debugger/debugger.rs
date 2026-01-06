use colored::Colorize;
use supports_color::Stream;

use star::prelude::*;

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

pub fn error_piece() -> String {
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

pub fn format_position_piece(s: String) -> String {
    let text = format!("[{}]", s);
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

fn info_piece() -> String {
    let text = "[info]".to_string();
    if let Some(color_level) = supports_color::on(Stream::Stdout) {
        if color_level.has_16m || color_level.has_256 {
            text
                .bold()
                .bright_cyan()
                .to_string()
        } else {
            text
        }
    } else {
        text
    }
}

pub fn message(msg: &str) {
    println!(
        "{} {}",
        interpreter(),
        msg,
    );
}

pub fn new_line() {
    print!("\n");
}

pub fn info_message(inf: &str) {
    println!(
        "{} {} {}",
        interpreter(),
        info_piece(),
        inf,
    );
}

pub fn exit_with_error(err: &str) {
    println!(
        "\n{} {} {}",
        interpreter(),
        error_piece(),
        err,
    );
    std::process::exit(0);
}


pub fn exit_with_positional_error(star: &Star, error: &String, position: StarPosition) {
    println!(
        "\n{} {} {} {}",
        interpreter(),
        error_piece(),
        error,
        format_position_piece(position.get_position_path(star)),
    );
    std::process::exit(0);
}

pub fn exit_with_optional_positional_error(star: &Star, error: &String, position: Option<StarPosition>) {
    if let Some(pos) = position {
        exit_with_positional_error(star, error, pos);
    } else {
        exit_with_error(error);
    }
}
