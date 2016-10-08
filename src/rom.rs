use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub struct Rom {
    pub current_op_index: usize,
    pub memory: Vec<u8>,
}



impl Rom {
    pub fn read_next_instruction(&mut self) -> u8 {
        let next_operation = match self.memory.get(self.current_op_index) {
            Some(value) => value,
            None => panic!("There was no operation at {} address", self.current_op_index)
        };

        self.current_op_index += 1;
        *next_operation
    }

    pub fn open_rom(path: &str) -> Rom {
        let game = Rom { 
            memory: read_rom_content(path),
            current_op_index: 0,
        };
        game
    }
}



fn read_rom_content(path: &str) -> Vec<u8> {
    let mut file = match File::open(&Path::new(path)) {
        Ok(file) => file,
        Err(error) => panic!("Can't open file at {}", error),
    };
    let mut memory_dump: Vec<u8> = Vec::new();
    file.read_to_end(&mut memory_dump);

    memory_dump
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
