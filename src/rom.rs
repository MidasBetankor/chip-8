pub fn open_rom(path: &str) {
    println!("Opening ROM at {}", path);
}
pub fn read_instruction() -> u16 {
    15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_rom_test() {
        open_rom("path: &str");
    }

    #[test]
    fn read_instruction_test() {
        assert_eq!(15, read_instruction());
    }
}
