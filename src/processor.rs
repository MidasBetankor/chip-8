use rom::Rom;
use opcodes::Opcode;

pub fn process_rom(program: Rom) {
    println!("Processing the ROM, the size is {} bytes", program.size);
    let instruction = program.read_next_instruction();
    match instruction {
        Opcode::ClearScreen => {
            println!("Clearing");
            clear_screen();
        },
        _ => println!("Unknown code"),
    }
}

fn clear_screen() {
    println!("Cleared screen");
}
