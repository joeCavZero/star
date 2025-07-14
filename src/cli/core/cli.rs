#[derive(Debug)]
pub struct Cli {
    pub version: bool,
    pub help: bool,
    pub file: Option<String>,
    pub binary_destiny: Option<String>,
    pub from_binary: Option<String>,
    pub symbol_table: bool,
    pub registers: bool,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            version: false,
            help: false,
            file: None,
            binary_destiny: None,
            from_binary: None,
            symbol_table: false,
            registers: false,
        }
    }
}