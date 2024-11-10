use nom::{branch::alt, error::ErrorKind, multi::many0, IResult};

// Misc Parsers
pub mod condition;
pub mod data;
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

pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        arithmetic::parse_arithmetic,
        branch::parse_branch,
        control::parse_control,
        data_transfer::parse_data_transfer,
        logical::parse_logical,
    ))(input)
}

pub fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(parse_instruction)(input)
}

pub fn test_expects_success<T: PartialEq + std::fmt::Debug, U: PartialEq + std::fmt::Debug>(
    input: T,
    expected_remaining: T,
    expected_output: U,
    tested_function: &dyn Fn(T) -> IResult<T, U>,
) {
    let result = tested_function(input);
    assert!(result.is_ok());
    let (input, output) = result.unwrap();
    assert_eq!(input, expected_remaining);
    assert_eq!(output, expected_output);
}

pub fn test_expects_error<T, U>(
    input: T,
    expected_error_kind: ErrorKind,
    tested_function: &dyn Fn(T) -> IResult<T, U>,
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
