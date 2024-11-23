use nom::{branch::alt, error::ErrorKind, multi::many0, IResult};

/**
    Input for bit parsers. The first element of the tuple is a byte slice of the
    data that needs to be parsed. The second element of the tuple is the offset
    into the slice where parsing begins.
*/
type BitInput<'a> = (&'a [u8], usize);

// Misc Parsers
pub mod condition;
pub mod register;

// Instruction Parsers
pub mod arithmetic;
pub mod branch;
pub mod control;
pub mod data_transfer;
pub mod logical;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Arithmetic(arithmetic::Arithmetic),
    Branch(branch::Branch),
    Control(control::Control),
    DataTransfer(data_transfer::DataTransfer),
    Logical(logical::Logical),
}

pub fn parse_instruction(input: BitInput) -> IResult<BitInput, Instruction> {
    alt((
        arithmetic::parse_arithmetic,
        branch::parse_branch,
        control::parse_control,
        data_transfer::parse_data_transfer,
        logical::parse_logical,
    ))(input)
}

pub fn parse_instructions(input: BitInput) -> IResult<BitInput, Vec<Instruction>> {
    many0(parse_instruction)(input)
}

#[allow(clippy::type_complexity)]
pub fn test_expects_success<
    T: PartialEq + std::fmt::Debug,
    U: PartialEq + std::fmt::Debug,
    V: PartialEq + std::fmt::Debug,
>(
    input: (&[T], U),
    expected_remaining: (&[T], U),
    expected_output: V,
    tested_function: &dyn Fn((&[T], U)) -> IResult<(&[T], U), V>,
) {
    let result = tested_function(input);
    assert!(result.is_ok());
    let (input, output) = result.unwrap();
    assert_eq!(input, expected_remaining);
    assert_eq!(output, expected_output);
}

#[allow(clippy::type_complexity)]
pub fn test_expects_error<T, U, V>(
    input: (&[T], U),
    expected_error_kind: ErrorKind,
    tested_function: &dyn Fn((&[T], U)) -> IResult<(&[T], U), V>,
) {
    let result = tested_function(input);
    assert!(result.is_err());
    match result {
        Err(nom::Err::Error(e)) => {
            assert_eq!(e.code, expected_error_kind);
        }
        _ => panic!("Expected {:?} Error.", expected_error_kind),
    }
}
