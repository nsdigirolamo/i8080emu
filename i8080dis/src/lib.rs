use std::{error::Error, fs::File, io::Read, path::Path};

use parsers::{parse_instructions, Instruction};

pub mod parsers;

pub fn disassemble_binary(path: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
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
