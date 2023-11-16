pub mod sorted_collection;
use std::io;
pub(crate) mod utils;
use sorted_collection::SortedCollection;
use utils::*;

mod encoding;

use serde_json::Value;

const VERSION: u8 = 0;

fn main() {
    let inputfile = std::env::args()
        .nth(1)
        .expect("first argument to be input json");
    let outputfile = std::env::args()
        .nth(2)
        .expect("second argument to be output file name");

    let reader = std::fs::File::open(inputfile).expect("read inputfile");
    let json: Value =
        serde_json::from_reader(io::BufReader::new(reader)).expect("read input json file");

    // get all strings inside the json document
    let strings = {
        let mut v = Vec::new();
        collect_string_values(&json, &mut v);
        v
    };

    // sort and deduplicate them
    let strings = SortedCollection::new(strings);

    // now, writing the result
    let mut writer = std::fs::File::create(outputfile).expect("create outputfile");

    compress_json(&json, &strings, &mut writer).expect("write compressed json");
}

fn compress_json(
    json: &Value,
    strings: &SortedCollection<&str>,
    w: &mut impl io::Write,
) -> io::Result<()> {
    // Write Version number
    w.write_all(&[VERSION])?;

    // Write prefix-encoded strings
    write_compressed_strings(strings, w)?;

    // now, write the actuall json structure.

    Ok(())
}

fn write_compressed_strings(
    strings: &SortedCollection<&str>,
    w: &mut impl io::Write,
) -> io::Result<()> {
    // write amount of strings
    let amount = strings.len();
    write_number(w, amount as u64)?;

    // return early, of no further work is to be done
    if strings.len() == 0 {
        return Ok(());
    }

    // the first string is special, because we can ommit the amount of prefix
    // write the length of the first string
    let fst = strings[0].as_bytes();
    let len = fst.len();
    write_number(w, len as u64)?;
    w.write_all(fst)?;

    let mut prev = fst;
    for s in strings.values().iter().skip(1) {
        let s = s.as_bytes();
        let cpl = common_prefix_len(prev, s);
        let len = s.len() - cpl;
        write_number(w, cpl as u64)?;
        write_number(w, len as u64)?;
        let bytes = &s[cpl..];
        w.write_all(bytes)?;
        prev = s;
    }

    Ok(())
}
