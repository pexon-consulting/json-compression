pub mod sorted_collection;
use sorted_collection::SortedCollection;
use std::io;

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
        encoding::utils::collect_string_values(&json, &mut v);
        v
    };

    // sort and deduplicate them
    let strings = SortedCollection::new(strings);

    // now, writing the result
    let mut writer = std::fs::File::create(outputfile).expect("create outputfile");

    compress_json(&json, &strings, &mut writer).expect("write compressed json");
}

fn compress_json(
    value: &Value,
    string_container: &SortedCollection<&[u8]>,
    w: &mut impl io::Write,
) -> io::Result<()> {
    // Write Version number
    w.write_all(&[VERSION])?;

    // Write prefix-encoded strings
    encoding::utils::write_compressed_strings(string_container, w)?;

    // now, write the actuall json structure.
    encoding::write_json(value, string_container, w)
}
