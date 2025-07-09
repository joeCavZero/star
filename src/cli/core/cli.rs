#[derive(Debug)]
pub struct Cli {
    pub version: bool,
    pub help: bool,
    pub base_file: Option<String>,
    pub binary_destiny: Option<String>,
    pub symbol_table: bool,
    pub registers: bool,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            version: false,
            help: false,
            base_file: None,
            binary_destiny: None,
            symbol_table: false,
            registers: false,
        }
    }
}