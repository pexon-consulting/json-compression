use std::io;
use crate::utils; 

/// Utilities for encoding json.

const NULL: u8 = 0;
const FALSE: u8 = 01;
const TRUE: u8 = 10;

fn write_null(w: &mut impl io::Write) -> io::Result<()> {
    w.write_all(&[NULL])
}

fn write_bool(w: &mut impl io::Write, value: bool) -> io::Result<()> {
    let code = if value { TRUE } else { FALSE };
    w.write_all(&[code])
}

fn write_int(w: &mut impl io::Write, value: i64) -> io::Result<()> {
    let code: u8 = 0b00_01_0000;
    let sign = if value >= 0 { 0 } else { 1 } << 3;

    // We're using the remaining 3 bytes to compress small numbers
    let is_small = value < 0b111 && value > -0b111;

    if is_small {
        let code = code | sign | (value & 0b111);
        return w.write_all(&[code]);
    }

    // number is to large, write in variable size. 

    let code = code |sign|0b111;
    w.write_all(&[code])?;

    utils::write_number(w, value.abs() as u64)
}

fn write_float(w: &mut impl io::Write, value: f64) -> io::Result<()> {
    // TODO stronger compression CAN be applied here.
    let code = 0b00_10_0000;
    let data = value.to_le_bytes();
    w.write_all(&[code])?;
    w.write_all(&data)
}