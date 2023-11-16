use std::{io, collections::HashSet};
use serde_json::{Value, Map};

use crate::{utils::{self, write_number}, sorted_collection::SortedCollection}; 

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

fn write_int(value: i64, w: &mut impl io::Write) -> io::Result<()> {
    let sign = if value >= 0 { 0 } else { 1 } << 3;

    // We're using the remaining 3 bytes to compress small numbers
    let is_small = value < 0b111 && value > -0b111;

    if is_small {
        let value = (value & 0b111) as u8;
        let code = INT | sign | value;
        return w.write_all(&[code]);
    }

    // number is to large, write in variable size. 

    let code = INT |sign|0b111;
    w.write_all(&[code])?;

    utils::write_number(w, value.unsigned_abs())
}

fn write_float(value: f64, w: &mut impl io::Write) -> io::Result<()> {
    // TODO stronger compression CAN be applied here.
    let data = value.to_le_bytes();
    w.write_all(&[FLOAT])?;
    w.write_all(&data)
}

fn write_string(value: &str, string_container: SortedCollection<&str>, w: &mut impl io::Write) -> io::Result<()> {
    let index = string_container.find(&value).expect("find string in string-collection");

    if index < 0b111111 {
        // if the index is small, we can put it in the remaining bits
        let index = index as u8;
        let code = STR | index;
        w.write_all(&[code])
    } else {
        // otherwise we place a marker (0b111111) to indicate that a vbyte encoded number follows.
        let code = STR | 0b111111;
        w.write_all(&[code])?;
        write_number(w, index as u64)
    }
}

fn write_array(value: &[Value], string_container: SortedCollection<&str>, w: &mut impl io::Write) -> io::Result<()> {
    todo!()
}

fn write_object(value: &Map<String, Value>, string_container: SortedCollection<&str>, w: &mut impl io::Write) -> io::Result<()> {
    todo!()
}

pub fn write_json(value: &Value, string_container: SortedCollection<&str>, w: &mut impl io::Write) -> io::Result<()> {
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
        },
        Value::String(value) => write_string(value, string_container, w),
        Value::Array(value) => write_array(value, string_container, w),
        Value::Object(value) => write_object(value, string_container, w),
    }
}