/*mod opcodes;

pub fn process_instruction(opcode: Operations) {
    match opcode {
        Operations::clear_screen => println!("0x00E0 that is"),
        //0 => panic!("{} is one invalid opcode", opcode),
        _ => println!("processing"),
    }

    opcodes::clear_screen();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_valid_instruction_test() {
        process_instruction(15);
    }

    #[test]
    #[should_panic]
    fn process_invalid_instruction_test() {
        process_instruction(0);
    }
}
#[path="../rom.rs"]
mod rom;

pub fn process_rom(program: Rom) {
    println!("loaded the ROM");
}
*/
