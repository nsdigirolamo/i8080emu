pub mod emu;
pub mod parsers;

use std::{env, fs::File, io::Read, path::Path};
use emu::State;

fn main() {
    let state = &mut Default::default();

    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();

    match command {
        "load" => load_program_to_memory(state, 0x10, args[2].as_str()),
        _ => println!("Invalid command."),
    }

    println!("{:?}", state.memory);
}

fn load_program_to_memory(state: &mut State, starting_addr: u16, path_to_program: &str) {
    let mut buffer = Vec::new();
    let _ = match File::open(Path::new(path_to_program)) {
        Ok(mut file) => file.read_to_end(&mut buffer),
        Err(e) => panic!("{}", e.to_string()),
    };

    if state.memory.len() < buffer.len() {
        panic!("The program is too large to load into memory.")
    }

    for (index, byte) in buffer.into_iter().enumerate() {
        state.set_memory(starting_addr + index as u16, byte);
    }
}
