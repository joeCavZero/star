#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Trinity,
    Hime,
    Pair,
    Clover,
    Ark,
}

impl Format {
    pub fn from_u16(fmt: u16) -> Self {

        if fmt & 0b_0000_1111_1111_1111 == 0b_0000_1111_1111_1111 {
            Format::Ark
        } else if fmt & 0b_0000_0000_1111_1111 == 0b_0000_0000_1111_1111 {
            Format::Clover
        } else if fmt & 0b_0000_0000_0000_1111 == 0b_0000_0000_0000_1111 {
            Format::Pair
        } else {
            let first_four_bits = fmt & 0b_0000_0000_0000_1111;
            if first_four_bits == 0b_0000_0000_0000_0111 || first_four_bits == 0b_0000_0000_0000_1000 {
                Format::Hime
            } else {
                Format::Trinity
            }
        }
        

    }
}