use opcodes::Opcode;

pub struct Rom {
    pub size: u16,
}

impl Rom {
    pub fn read_next_instruction(&self) -> Opcode {
        Opcode::ClearScreen
    }
}

pub fn open_rom(path: &str) -> Rom {
    println!("Opening ROM at {}", path);
    Rom { size : 10 }
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
