mod rom;
mod processor;
mod opcodes;

use rom::Rom;

fn main() {
    println!("Boot!");
    let game: Rom = rom::open_rom("BRIX");
    processor::process_rom(game);
    println!("YOU DIED");
}
