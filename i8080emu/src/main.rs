// enum Register {
//     /** Accumulator */
//     A = 0b111,
//     B = 0b000,
//     C = 0b001,
//     D = 0b010,
//     E = 0b011,
//     H = 0b100,
//     L = 0b101,
// }

// enum RegisterPair {
//     BC = 0b00,
//     DE = 0b01,
//     HL = 0b10,
//     /** Stack Pointer */
//     SP = 0b11,
// }

// enum ConditionFlag {
//     /** If the result of an instruction has the value 0, this flag is set;
//     otherwise it is reset. */
//     Zero,

//     /** If the most significant bit of the result of the operation has the value
//     1, this flag is set; otherwise it is reset. */
//     Sign,

//     /** If the modulo 2 sum of the bits of the result of the operation is 0,
//     (i.e., if the result has even parity), this flag is set; otherwise it is
//     reset (i.e., if the result has odd parity). */
//     Parity,

//     /** If the instruction resulted in a carry (from addition), or a borrow
//     (from subtraction or a comparison) out of the high-order bit, this flag is
//     set; otherwise it is reset. */
//     Carry,

//     /** If the instruction caused a carry out of bit 3 and into bit 4 of the
//     resulting value, the auxiliary carry is set; otherwise it is reset. This
//     flag is affected by single precision additions, subtractions, increments,
//     decrements, comparisons, and logical operations, but is principally used
//     with additions and increments preceding a Decimal Adjust Accumulator (DAA)
//     instruction. */
//     AuxiliaryCarry,
// }

fn main() {
    let num1 = 3;
    let num2 = 5;
    let result = i8080dis::add(num1, num2);
    println!("Hello, world! {num1} plus {num2} equals {result}");
}
