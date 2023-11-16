use serde_json::Value;
use std::io;

/// Write a vbyte encoded number into a buffer
pub fn write_number(w: &mut impl io::Write, value: u64) -> io::Result<()> {
    let data = vbyte::compress(value);
    w.write_all(&data)?;
    Ok(())
}

pub fn common_prefix_len(s1: &[u8], s2: &[u8]) -> usize {
    s1.iter().zip(s2).take_while(|(a, b)| a == b).count()
}

pub fn collect_string_values<'a>(v: &'a Value, b: &mut Vec<&'a str>) {
    match v {
        Value::String(s) => {
            b.push(s);
        }
        Value::Array(a) => {
            for entry in a {
                collect_string_values(entry, b);
            }
        }
        Value::Object(o) => {
            for (key, value) in o {
                b.push(key);
                collect_string_values(value, b)
            }
        }
        _ => {}
    }
}
