use io;
use std::{collections::BTreeSet, io};
pub(crate) mod utils;
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
    let mut strings = {
        let mut v = Vec::new();
        collect_string_values(&json, &mut v);
        v
    };

    // sort them
    let strings: BTreeSet<_> = strings.into_iter().collect();

    // now, writing the result
    let mut writer = std::fs::File::create(outputfile).expect("create outputfile");

    compress_json(&json, &strings, &mut writer);
}

fn compress_json(json: &Value, strings: &BTreeSet<&str>, w: &mut impl io::Write) -> _ {
    // Write Version number
    w.write(&[VERSION])?;

    // Write prefix-encoded strings
    write_compressed_strnigs(strings, w)?;

    // now, write the actuall json structure.
}

fn write_compressed_strings(strings: &BTreeSet<&str>, w: &mut impl io::Write) -> io::Result<()> {
    // write amount of strings
    let amount = strings.len();  
    write_number(w, amount)?;
    
    // return early, of no further work is to be done 
    if strings.len() == 0 {
        return Ok(());
    }
    
    // the first string is special, because we can ommit the amount of prefix
    // write the length of the first string
    let fst = strings[0].as_bytes();
    let len = fst.len();
    write_number(w, len)?;
    w.write_all(fst);


    let mut prev = fst;
    for s in strings.iter().skip(1) {
        let s = s.as_bytes();
        let cpl = common_prefix_len(prev, s);
        let len = s.len() - cpl;
        write_numer(w, cpl)?;
        write_numer(w, len)?;
        let bytes = &s[cpl..];
        w.write_all(bytes)?;
        prev = s;
    }


    Ok(())
}