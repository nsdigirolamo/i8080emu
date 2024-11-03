use parsers::data_transfer::mov::parse_move_register;

pub mod parsers;
#[cfg(test)]
pub mod tests;

pub fn disassemble_binary(filename: &str) {
    // let bytes: Vec<u8> = std::fs::read(filename).expect("File not found");
    // let mut input: Vec<String> = Vec::new();
    // for byte in bytes {
    //     input.push(format!("{:08b}", byte))
    // }
    // print!("{:?}", input)

    let instruction = "01111101";
    let (_, result) = parse_move_register(instruction).expect("Failure!");
    println!("{filename}: {:?}", result);
}
