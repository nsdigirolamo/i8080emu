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
            eprintln!("{:02X?}", state.memory);
            state.start();
        }
        _ => println!("Invalid command."),
    }
}
