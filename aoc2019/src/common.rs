use std::io::{self,prelude::*};
use std::fs;

const DATA_DIR: &str = "/home/bjg/Projects/aoc2019/data";
// const DATA_DIR: &str = "/Users/brian.gillespie/Rust/AdventOfCode2019/data";


pub fn file_to_string(file_path: &str) -> String {
    let file_path = format!("{}/{}", DATA_DIR, file_path);
    // println!("{}", file_path);
    let file = fs::File::open(&file_path).expect("Couldn't open file");
    let mut br = io::BufReader::new(file);
    let mut buf = String::new();
    br.read_to_string(&mut buf).expect("Couldn't read file");
    buf
}
