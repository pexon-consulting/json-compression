use serde_json::Value;
use std::io;

use crate::sorted_collection::SortedCollection;

pub fn common_prefix_len(s1: &[u8], s2: &[u8]) -> usize {
    s1.iter().zip(s2).take_while(|(a, b)| a == b).count()
}

/// Write a vbyte encoded number into a buffer
pub fn write_number(value: u64, w: &mut impl io::Write) -> io::Result<()> {
    let data = vbyte::compress(value);
    w.write_all(&data)?;
    Ok(())
}

pub fn collect_string_values<'a>(v: &'a Value, b: &mut Vec<&'a [u8]>) {
    match v {
        Value::String(s) => {
            b.push(s.as_bytes());
        }
        Value::Array(a) => {
            for entry in a {
                collect_string_values(entry, b);
            }
        }
        Value::Object(o) => {
            for (key, value) in o {
                b.push(key.as_bytes());
                collect_string_values(value, b)
            }
        }
        _ => {}
    }
}

pub fn write_compressed_strings(
    strings: &SortedCollection<&[u8]>,
    w: &mut impl io::Write,
) -> io::Result<()> {
    // write amount of strings
    let amount = strings.len();
    write_number(amount as u64, w)?;

    // return early, of no further work is to be done
    if strings.len() == 0 {
        return Ok(());
    }

    // the first string is special, because we can ommit the amount of prefix
    // write the length of the first string
    let fst = strings[0];
    let len = fst.len();
    write_number(len as u64, w)?;
    w.write_all(fst)?;

    let mut prev = fst;
    for s in strings.values().iter().skip(1) {
        let cpl = common_prefix_len(prev, s);
        let len = s.len() - cpl;
        write_number(cpl as u64, w)?;
        write_number(len as u64, w)?;
        let bytes = &s[cpl..];
        w.write_all(bytes)?;
        prev = s;
    }

    Ok(())
}
