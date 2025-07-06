pub trait Stringable {
    fn processed_string(&self) -> String;
    fn beautiful_path(&self) -> String;
}

impl Stringable for String {
    fn processed_string(&self) -> String {
        let mut result = String::new();
        let mut chars = self.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('"') => result.push('"'),
                    Some('\\') => result.push('\\'),
                    Some('0') => result.push('\0'),
                    Some(ch) => {
                        result.push('\\');
                        result.push(ch);
                    },
                    None => {
                        result.push('\\');
                    },
                }
            } else {
                result.push(ch);
            }
        }
        
        result
    }

    fn beautiful_path(&self) -> String {
        // e.g.: \\?\D:\codigos\rust\star-vm\test4.asm -> D:/codigos/rust/star-vm/test4.asm
        self
            .replace("\\\\?\\", "")
            .replace("\\", "/")
            .replace("//", "/")
            .trim_end_matches('/')
            .to_string()
    }
}