use crate::split_u16;

use super::State;

/**
    Increase address. Adds one to the argument then returns.
*/
macro_rules! inca {
    ($addr:expr) => {{
        $addr += 1;
        $addr
    }};
}

/**
    Loads the implementation for the basic disk operating system (BDOS) function
    calls into memory. System function calls are performed by placing the
    function code into register C and then making a `CALL` to `0x0005`.
*/
pub fn load_bdos_functions(state: &mut State) {
    let func2_start = 0x0016;
    let func2_end = load_console_output(state, func2_start);
    let func9_start = func2_end + 1;
    load_print_string(state, func9_start);

    state.memory[0x0005] = 0b11100110; // ANI 0 - Clear accumulator by AND-ing with zero.
    state.memory[0x0006] = 0b00000000;
    state.memory[0x0007] = 0b11000110; // ADI 2 - Add 2 into the accumulator.
    state.memory[0x0008] = 0b00000010;
    state.memory[0x0009] = 0b10111001; // CMP C - Compare accumulator with C.
    state.memory[0x000A] = 0b11001010; // Jcondition - Conditional jump; if C equalled 2 then jump.
    let (high_addr, low_addr) = split_u16!(func2_start);
    state.memory[0x000B] = low_addr; // Jump to address of function 2.
    state.memory[0x000C] = high_addr;

    state.memory[0x000D] = 0b11100110; // ANI 0 - Clear accumulator by AND-ing with zero.
    state.memory[0x000E] = 0b00000000;
    state.memory[0x000F] = 0b11000110; // ADI 9 - Add 9 into the accumulator.
    state.memory[0x0010] = 0b00001001;
    state.memory[0x0011] = 0b10111001; // CMP C - Compare accumulator with C.
    state.memory[0x0012] = 0b11001010; // Jcondition - Conditional jump; if C equalled 9 then jump.
    let (high_addr, low_addr) = split_u16!(func9_start);
    state.memory[0x0013] = low_addr; // Jump to address of function 9.
    state.memory[0x0014] = high_addr;

    state.memory[0x0015] = 0b01110110; // Halt if the function code isn't supported.
}

/**
    Loads the implementation for BDOS Function 2 into memory starting at
    `starting_addr`. The Console Output function sends the ASCII character from
    register E to the standard output.

    Returns the memory address of the final instruction of the function.
*/
fn load_console_output(state: &mut State, starting_addr: u16) -> u16 {
    let mut a = starting_addr as usize;
    state.memory[a] = 0b11100110; // ANI 0 - Clear accumulator by AND-ing with zero.
    state.memory[inca!(a)] = 0b00000000;
    state.memory[inca!(a)] = 0b10110011; // OR E - OR accumulator with E to set it to E's value.
    state.memory[inca!(a)] = 0b11010011; // OUT 0 - Output data stored in accumulator.
    state.memory[inca!(a)] = 0b00000000;
    state.memory[inca!(a)] = 0b11001001; // RET - Return.
    a as u16
}

/**
    Loads the implementation for BDOS Function 9 into memory starting at
    `starting_addr`. The Print String function sends the character string
    addressed by register pair DE to standard output until it encounters the
    `$` delimiter.

    Returns the memory address of the final instruction of the function.
*/
fn load_print_string(state: &mut State, starting_addr: u16) -> u16 {
    let mut a = starting_addr as usize;
    state.memory[a] = 0b00011010; // LDAX BC - Load the content of the memory address in BC into accumulator.
    state.memory[inca!(a)] = 0b11111110; // CPI 36 - Compare accumulator with value of '$'.
    state.memory[inca!(a)] = 0b00100100;
    state.memory[inca!(a)] = 0b11001000; // Rcondition - Conditional return; if the accumulator stored '$' then return.
    state.memory[inca!(a)] = 0b11010011; // OUT 0 - Otherwise, output data stored in accumulator.
    state.memory[inca!(a)] = 0b00000000;
    state.memory[inca!(a)] = 0b00010011; // INX BC - Increment address in BC.
    state.memory[inca!(a)] = 0b11000011; // JMP 0x0005 - Jump back to beginning of the routine.
    state.memory[inca!(a)] = 0b00000101;
    state.memory[inca!(a)] = 0b00000000;
    a as u16
}
