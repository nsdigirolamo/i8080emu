pub mod emu;
pub mod parsers;

use emu::State;
use std::{env, fs::File, io::Read, path::Path};

fn main() {
    let state = &mut Default::default();

    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();

    match command {
        "load" => load_program_to_memory(state, 16, args[2].as_str()),
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

    let address_size = (state.memory.len() as u16 - starting_addr) as usize;
    if address_size < buffer.len() {
        panic!("The program is too large to load into memory.")
    }

    for (index, data) in buffer.into_iter().enumerate() {
        let address = starting_addr + index as u16;
        state.set_memory(address, data);
    }
}
