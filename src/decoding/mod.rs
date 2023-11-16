pub mod utils;
use std::ops::Add;

use nom::{bytes::complete::{ tag, take }, combinator::map, branch::alt, Err};
use serde_json::{Map, Value};

use crate::encoding::{NULL, FALSE, TRUE, INT, FLOAT};

use self::utils::inplace_number;

/// Utilities for encoding json.

const INT_MARKER_MASK: u8 = 0b1111_0000;
const INT_SIGN_MASK: u8 = 0b00_00_10_00;

fn parse_null(data: &[u8]) -> nom::IResult<&[u8], Value> {
    map(tag(&[NULL]), |_| Value::Null)(data)
}

fn parse_bool(data: &[u8]) -> nom::IResult<&[u8], Value> {
    alt((
        map(tag(&[FALSE]), |_| Value::Bool(false)),
        map(tag(&[TRUE]), |_| Value::Bool(true)),
    ))(data)
}

/// parses an compressed integervalue
fn parse_int(data: &[u8]) -> nom::IResult<&[u8], i64> {
    let (rest, (code, value)) = inplace_number(5, data)?;
    let is_positive = (code & INT_SIGN_MASK) == 0;

    let code = code & INT_MARKER_MASK; 
    if code != INT {
        return Err(Err::Error(
            nom::error::Error { input: data, code: nom::error::ErrorKind::TagBits }
        ));
    }


    if is_positive {
        Ok((rest, value as i64))
    } else {
        let value = -(value as i64).add(1);
        Ok((rest, value))
    }
}

fn eight_byte_slice(data: &[u8]) -> [u8; 8] {
    let mut buffer = [0; 8];
    buffer.copy_from_slice(data);

    buffer
}

fn parse_float(data: &[u8]) -> nom::IResult<&[u8], f64> {
    let (rest, _) = tag(&[FLOAT])(data)?;
    let (rest, bytes) = take(8usize)(rest)?;
    let value = f64::from_le_bytes(eight_byte_slice(bytes));

    Ok((rest, value))
}

fn parse_string<'a>(
    string_container: &[String],
    data: &'a [u8],
) -> nom::IResult<&'a [u8], String> {
    unimplemented!()
}

fn array<'a>(
    string_container: &[String],
    data: &'a [u8],
) -> nom::IResult<&'a [u8], Vec<Value>> {
    unimplemented!()
}

fn object<'a>(
    string_container: &[String],
    data: &'a [u8],
) -> nom::IResult<&'a [u8], Map<String, Value>> {
    unimplemented!()
}

pub fn json<'a>(
    string_container: &[String],
    data: &'a [u8],
) -> nom::IResult<&'a [u8], Value> {
    unimplemented!()
}
