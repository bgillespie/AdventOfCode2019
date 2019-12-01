use std::io::{self,prelude::*};
use std::fs;

const DATA_DIR: &str = "/home/bjg/Projects/aoc2019/data/";


pub fn file_to_string(file_path: &str) -> String {
    let file = fs::File::open(&format!("{}/{}", DATA_DIR, file_path)).expect("Couldn't open file");
    let mut br = io::BufReader::new(file);
    let mut buf = String::new();
    br.read_to_string(&mut buf).expect("Couldn't read file");
    buf
}
