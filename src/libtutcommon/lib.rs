#![deny(deprecated)]
#![deny(missing_docs)]

#![doc = "Common stuff for tutorials."]
#![crate_name = "tutcommon"]

use std::fs::File;
use std::io::Read;
use std::path::AsPath;

#[doc = "Read content of file into string."]
pub fn read_source_from_file<P: AsPath + ?Sized>(path : &P) -> String {
    let mut res = String::new();
    File::open(path).unwrap().read_to_string(&mut res).unwrap();
    return res;
}

