fn main() {
    let instructions =
        i8080dis::disassemble_binary("/home/nick/dev/i8080emu/cpu_tests/8080PRE.COM");
    println!("{:#?}", instructions);
}
