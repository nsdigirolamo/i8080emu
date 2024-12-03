use std::env;

use emu::State;

pub mod emu;
pub mod parsers;

fn main() {
    let state: &mut State = &mut Default::default();

    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();

    match command {
        "run" => {
            state.load_program(args[2].as_str());
            state.start();
            println!();
        }
        _ => println!("Invalid command."),
    }
}

/*

0xE60E00

0b1110_0110_0000_1110_0000_0000 -> ANI 0b0000_1110 -> ANI 0x0E -> ANI 14

Register F -> 0x02 -> 0b0000_0010 -> S0 Z0 0 AC0 0 P0 1 CY0
Should be  -> 0x12 -> 0b0001_0010 -> S0 Z0 0 AC1 0 P0 1 CY0

reg before instruction was 0x06 -> 0b0000_0110

0b0000_1110
0b0000_0110
0b0000_0110

 */
