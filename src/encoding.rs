use crate::{sorted_collection::SortedCollection, utils};
use std::{collections::HashSet, io};

/// Utilities for encoding json.

const NULL: u8 = 0;
const FALSE: u8 = 0b01;
const TRUE: u8 = 0b10;
const FLOAT: u8 = 0b00_10_0000;
const INT: u8 = 0b00_01_0000;

pub fn write_null(w: &mut impl io::Write) -> io::Result<()> {
    w.write_all(&[NULL])
}

pub fn write_bool(w: &mut impl io::Write, value: bool) -> io::Result<()> {
    let code = if value { TRUE } else { FALSE };
    w.write_all(&[code])
}

pub fn write_int(w: &mut impl io::Write, value: i64) -> io::Result<()> {
    let sign = if value >= 0 { 0 } else { 1 } << 3;

    // We're using the remaining 3 bytes to compress small numbers
    let is_small = value < 0b111 && value > -0b111;

    if is_small {
        let value = (value & 0b111) as u8;
        let code = INT | sign | value;
        return w.write_all(&[code]);
    }

    // number is to large, write in variable size.

    let code = INT | sign | 0b111;
    w.write_all(&[code])?;

    utils::write_number(w, value.abs() as u64)
}

pub fn write_float(w: &mut impl io::Write, value: f64) -> io::Result<()> {
    // TODO stronger compression CAN be applied here.
    let data = value.to_le_bytes();
    w.write_all(&[FLOAT])?;
    w.write_all(&data)
}

pub fn write_string(
    value: &str,
    string_container: SortedCollection<&str>,
    w: &mut impl io::Write,
) -> io::Result<()> {
    todo!()
}
