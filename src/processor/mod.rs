mod opcodes;

pub fn process_instruction(opcode: u16) {
    match opcode {
        0 => panic!("{} is one invalid opcode", opcode),
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
