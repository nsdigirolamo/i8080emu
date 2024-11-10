use std::env;

use i8080dis::disassemble_binary;

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
