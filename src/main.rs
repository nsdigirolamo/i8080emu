use std::env;

use emu::State;

pub mod emu;
pub mod parsers;

fn main() {
    let state: &mut State = &mut Default::default();

    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();

    match command {
        "load" => state.load_program(16, args[2].as_str()),
        "start" => state.start(),
        _ => println!("Invalid command."),
    }

    println!("{:?}", state.memory);
}
