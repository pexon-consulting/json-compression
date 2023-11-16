use serde_json::{Map, Value};
use std::{collections::HashSet, io, ops::Sub};

use crate::{
    sorted_collection::SortedCollection,
    utils::{self, write_number},
};

/// Utilities for encoding json.

const NULL: u8 = 0;
const FALSE: u8 = 0b01;
const TRUE: u8 = 0b10;
const INT: u8 = 0b00_01_00_00;
const FLOAT: u8 = 0b00_10_00_00;

const STR: u8 = 0b01_00_00_00;
const ARR: u8 = 0b10_00_00_00;
const OBJ: u8 = 0b11_00_00_00;

fn write_null(w: &mut impl io::Write) -> io::Result<()> {
    w.write_all(&[NULL])
}

fn write_bool(value: bool, w: &mut impl io::Write) -> io::Result<()> {
    let code = if value { TRUE } else { FALSE };
    w.write_all(&[code])
}

/// Utility to use the avaible set of unused bits after the code
/// to possibly encode a number inplace
/// and alternatively place a marker to indicate the number gets written afterwards
fn write_inplace_number(
    code: u8,
    available_bits: u8,
    value: u64,
    w: &mut impl io::Write,
) -> io::Result<()> {
    assert!(available_bits != 0);

    let mask = (1 << available_bits) - 1;

    if value < (mask as u64) {
        // the number is small enough to be saved inplace
        let value = value as u8 & mask;
        let code = code | value;
        w.write_all(&[code])
    } else {
        let code = code | mask;
        w.write_all(&[code])?;
        write_number(value, w)
    }
}

fn write_int(value: i64, w: &mut impl io::Write) -> io::Result<()> {
    let sign = if value >= 0 { 0 } else { 1 } << 3;
    let code = INT | sign;
    let mut value = if value >= 0 {
        value as u64
    } else {
        // value is at least -1.
        // to avoid redundancy, we're decrementing this number here by one.
        (-value).sub(1) as u64
    };

    // we have 3 unused bits available that may be used to store small numbers.
    let available_bits = 3;

    write_inplace_number(code, available_bits, value, w)
}

fn write_float(value: f64, w: &mut impl io::Write) -> io::Result<()> {
    // TODO stronger compression CAN be applied here.
    let data = value.to_le_bytes();
    w.write_all(&[FLOAT])?;
    w.write_all(&data)
}

fn write_string(
    value: &[u8],
    string_container: &SortedCollection<&[u8]>,
    w: &mut impl io::Write,
) -> io::Result<()> {
    // we only need to write down the index, as the content of the string is saved at the beginning of the document.
    let index = string_container
        .find(&value)
        .expect("find string in string-collection");

    // write the marker to indicate strings.
    // write the length of the string.
    write_inplace_number(STR, 6, index as u64, w)
}

fn write_array(
    value: &[Value],
    string_container: &SortedCollection<&[u8]>,
    w: &mut impl io::Write,
) -> io::Result<()> {
    // First, we encode the length
    let length = value.len();

    write_inplace_number(ARR, 6, length as u64, w)?;

    // now we write all the elements of the array down

    for v in value {
        write_json(v, string_container, w)?;
    }

    Ok(())
}

fn write_object(
    value: &Map<String, Value>,
    string_container: &SortedCollection<&[u8]>,
    w: &mut impl io::Write,
) -> io::Result<()> {
    // First, we write how many entries do follow.
    let length = value.len();
    write_inplace_number(ARR, 6, length as u64, w)?;

    // then we write the down the keys and values one by one

    for (k, v) in value {
        write_string(k.as_bytes(), string_container, w)?;
        write_json(v, string_container, w)?;
    }

    Ok(())
}

pub fn write_json(
    value: &Value,
    string_container: &SortedCollection<&[u8]>,
    w: &mut impl io::Write,
) -> io::Result<()> {
    match value {
        Value::Null => write_null(w),
        Value::Bool(value) => write_bool(*value, w),
        Value::Number(value) => {
            // while json does not differentiate between integers and floats
            // for compression it is useful to do so.
            if let Some(value) = value.as_i64() {
                write_int(value, w)
            } else if let Some(value) = value.as_f64() {
                write_float(value, w)
            } else {
                panic!("failed to write number {value:?}. Not an i64 or f64")
            }
        }
        Value::String(value) => write_string(value.as_bytes(), string_container, w),
        Value::Array(value) => write_array(value, string_container, w),
        Value::Object(value) => write_object(value, string_container, w),
    }
}
