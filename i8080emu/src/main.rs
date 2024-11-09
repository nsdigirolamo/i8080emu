use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args[1].as_str();

    match command {
        "dis" => {
            let instructions = i8080dis::disassemble_binary(args[2].as_str());
            println!("{:#?}", instructions);
        }
        _ => println!("Invalid command."),
    }
}
