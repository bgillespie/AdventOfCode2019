use std::io::{self,BufRead};
use std::str::FromStr;
use std::fs;
use std::path::{Path,PathBuf};
use std::env;
use std::fmt::Debug;

const DATA_DIR_ENV_VAR: &str = "AOC2019_DATA";
const DATA_DIR: &str = "/home/bjg/Projects/aoc2019/data/day";

#[derive(Debug)]
pub enum Error {
    DataDirEnvVarNotSet,
    DayDirNotRelative,
    DataFileNotRelative,
    IoError(io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IoError(e)
    }
}

/// Use env-var for root
fn data_root() -> Result<PathBuf,Error> {
    match env::var(DATA_DIR_ENV_VAR) {
        Err(_) => Err(Error::DataDirEnvVarNotSet),
        Ok(val) => Ok(PathBuf::from_str(DATA_DIR).unwrap())
    }
}

/// Get path to a data file from standard location
pub fn path_to_data_file(day: &str, file: &str) 
-> Result<PathBuf,Error> {
    let day_path = PathBuf::from_str(day).unwrap();
    let file_path = PathBuf::from_str(file).unwrap();
    if day_path.is_absolute() {
        Err(Error::DayDirNotRelative)
    }
    else if file_path.is_absolute() {
        Err(Error::DataFileNotRelative)
    }
    else {
        let mut path = data_root()?;
        path.push(day_path);
        path.push(file_path);
        println!("{:?}", path);
        Ok(path)
    }
}

/// Get path to input file for a given day
fn path_to_input(day: &str) -> Result<PathBuf,Error> {
    path_to_data_file(day, "input")
}

pub fn data_lines(path: &PathBuf)
-> Box<dyn Iterator<Item=Result<String, impl Into<Error>>>>
{
    let file = fs::File::open(path).expect("Couldn't open file");
    let mut br = io::BufReader::new(file);
    Box::new(br.lines())
}

pub fn data_file_to_string(day: &str, file: &str) -> Result<String, Error> {
    let path = path_to_data_file(day, file)?;
    let mut s = String::new();
    for line in data_lines(&path) {
        match line {
            Ok(l) => s.push_str(&l),
            Err(e) => return Err(e.into()),
        };
    }
    Ok(s)
}


/// Greatest Common Divisor
pub fn gcd(mut m: isize, mut n: isize) -> isize {
    while m != 0 {
        let temp = m;
        m = n % temp;
        n = temp;
    }
    if n == 0 {
        1
    }
    else {
        n.abs()
    }
}
