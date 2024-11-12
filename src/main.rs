pub mod emu;
pub mod parsers;

use std::{env, error::Error, fs::File, io::Read, path::Path};

use parsers::{parse_instructions, Instruction};

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args[1].as_str();

    match command {
        "dis" => match disassemble_binary(args[2].as_str()) {
            Ok(instructions) => println!("{:#?}", instructions),
            Err(e) => println!("{}", e.to_string()),
        },
        _ => println!("Invalid command."),
    }
}

fn disassemble_binary(path: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let mut file = File::open(Path::new(path))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let binary_string = buffer
        .iter()
        .map(|byte| format!("{:08b}", byte))
        .collect::<Vec<String>>()
        .join("");

    match parse_instructions(&binary_string) {
        Ok((_, instructions)) => Ok(instructions),
        Err(e) => Err(Box::new(e.to_owned())),
    }
}
