mod processor;
mod rom;

use rom::open_rom;

fn main() {
    println!("Works!");
    open_rom("BRIX");
    rom::read_instruction();
    processor::process_instruction(15);
}
