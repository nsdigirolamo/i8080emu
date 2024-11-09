use std::{fs::File, io::Read, path::Path};

use parsers::{parse_instructions, Instruction};

pub mod parsers;

pub fn disassemble_binary(path: &str) -> Vec<Instruction> {
    let mut file = File::open(Path::new(path)).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let binary_string = buffer
        .iter()
        .map(|byte| format!("{:08b}", byte))
        .collect::<Vec<String>>()
        .join("");

    let (_, instructions) = parse_instructions(&binary_string).unwrap();
    return instructions;
}
