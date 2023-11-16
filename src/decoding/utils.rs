/*
 * utility functions for encoding
 */

use std::ops::Sub;

use nom::bytes::complete::take;


/// Utility to use the avaible set of unused bits after the code
/// to possibly encode a number inplace
/// and alternatively place a marker to indicate the number gets written afterwards
pub fn inplace_number(
    bits_for_code: u8,
    data: &[u8],
) -> nom::IResult<&[u8], (u8, u64)> {
    assert!(bits_for_code != 0);

    // the code mask is `n = bits_for_code` 1s at the significant positions. 
    let code_mask: u8 = (1<<bits_for_code).sub(1u8) << (8u8 - bits_for_code);
    let value_mask = !code_mask;
    let (rest, fst_byte) = take(1usize)(data)?;
    let fst_byte = fst_byte[0];

    let code = fst_byte & code_mask;
    let value = fst_byte & value_mask;
    
    if value | code_mask == 0xff {
        // the value is large, and was stored appended to the other number
        let (rest, value) = vbyte_number(rest)?;
        Ok((rest, (code, value)))
    } else {
        // the value is so small, that it was stored in the remaining bits of the number
        Ok((rest, (code, value as u64)))
    }
}


/// parse a vbyte encoded number from data
pub fn vbyte_number(data: &[u8]) -> nom::IResult<&[u8], u64> {
    let (result, rest) = vbyte::decompress(data).map_err(|_| nom::Err::Incomplete(nom::Needed::Unknown))?;

    Ok((rest, result))
}

/// Decompresses a prefix encoded list of strings at the beginnngin of json compression format
pub fn decompress_strings(
    data: &[u8],
) -> nom::IResult<&[u8], Vec<String>> {
    let (rest, amount) = vbyte_number(data)?;
    if amount == 0 {
        return Ok((rest, Vec::new()));
    }

    let (rest, fstlen) = vbyte_number(data)?;
    let (rest, fstdata) = take(fstlen)(rest)?;
    let fst = String::from_utf8(fstdata.to_vec()).expect("bytes to be valid utf8");

    let mut strings = vec![fst];
    let mut data = rest;

    for _ in 1..amount {
        let (rest, cpl) = vbyte_number(data)?;
        let (rest, remaining_len) = vbyte_number(rest)?;
        let cpl = cpl as usize;
        let remaining_len = remaining_len as usize;

        let mut s = Vec::with_capacity(cpl + remaining_len);
        s.extend(&strings.last().unwrap().as_bytes()[..cpl]);
        let (rest, strdata) = take(remaining_len)(rest)?;
        s.extend(strdata);
        let s = String::from_utf8(s).expect("bytes to be valid utf8");

        // update cursor
        strings.push(s);
        data = rest;
    }

    Ok((rest, strings))
}