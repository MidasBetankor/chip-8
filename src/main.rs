mod rom;
mod processor;
mod opcodes;

use processor::Chip;
use rom::Rom;

fn main() {
    println!("Boot!");
    let mut game: Rom = Rom::open_rom("roms/BRIX");
    Chip::process_rom(&mut game);

    println!("YOU DIED");
}