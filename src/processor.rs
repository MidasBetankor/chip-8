use rom::Rom;

pub struct Chip {
    registers: [u8; 16]
}

impl Chip {
    pub fn process_rom(program: &mut Rom) {
        println!("Processing the ROM, the size is {} bytes", program.memory.len());
        let first_byte = program.read_next_instruction();
        match first_byte {
            0x00 => println!("Not supported"),
            0x0E => println!("qwe"),

            _ => println!("Unknown code"),
        }
    }
    
    fn clear_screen() {
        println!("Cleared screen");
    }
}


