pub struct Opcode {
    pub first_byte: u8,
    pub second_byte: u8,
    pub third_byte: u8,
    pub fourth_byte: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_screen_test() {
        clear_screen();
    }
}
