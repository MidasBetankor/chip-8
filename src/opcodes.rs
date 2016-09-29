pub enum Opcode {
    ClearScreen = 0x00E0,
    ReturnSubroutine = 0x00EE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_screen_test() {
        clear_screen();
    }
}
