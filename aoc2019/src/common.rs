use std::io::{self,BufRead};
use std::str::FromStr;
use std::fs;
use std::path::PathBuf;
use std::env;
use std::fmt::{self,Debug};
use std::ops::{AddAssign,SubAssign,Sub};
use std::cmp::Ordering;

const DATA_DIR_ENV_VAR: &str = "AOC2019_DATA";
pub const F32_RAD_TO_DEG: f32 = 180f32 / std::f32::consts::PI;

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
        Ok(val) => Ok(PathBuf::from_str(&val).unwrap())
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


//
//
//


/// Iterator over permutations of two digits up to a max.
pub struct TwoPermute {
    max    : usize,
    index_a: usize,
    index_b: usize,
}

impl TwoPermute {
    pub fn new(max: usize) -> Self {
        TwoPermute { max, index_a: 0, index_b: 1 }
    }
}

impl std::iter::Iterator for TwoPermute {
    type Item = (usize, usize);
    
    fn next(&mut self) -> Option<(usize, usize)> {
        if self.index_b > self.max {
            if self.index_a == self.max - 1 {
                None
            }
            else {
                self.index_a += 1;
                self.index_b = self.index_a + 2;
                Some((self.index_a, self.index_b - 1))
            }
        }
        else {
            self.index_b += 1;
            Some((self.index_a, self.index_b - 1))
        }
    }
}

#[test]
fn test_two_permute() {
    let mut tp = TwoPermute::new(4);
    assert_eq!(tp.next(), Some((0,1)));
    assert_eq!(tp.next(), Some((0,2)));
    assert_eq!(tp.next(), Some((0,3)));
    assert_eq!(tp.next(), Some((0,4)));
    assert_eq!(tp.next(), Some((1,2)));
    assert_eq!(tp.next(), Some((1,3)));
    assert_eq!(tp.next(), Some((1,4)));
    assert_eq!(tp.next(), Some((2,3)));
    assert_eq!(tp.next(), Some((2,4)));
    assert_eq!(tp.next(), Some((3,4)));
    assert_eq!(tp.next(), None);
}


//
// V3
//

#[derive(Clone,Debug,Copy,PartialEq,Eq,Hash)]
pub struct V3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Ord for V3 {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = self.z.cmp(&other.x);
        let y = self.z.cmp(&other.y);
        let z = self.z.cmp(&other.z);
        if z != Ordering::Equal {
            z
        }
        else if x != Ordering::Equal {
            x
        }
        else {
            y
        }
    }
}

impl PartialOrd for V3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl AddAssign for V3 {
    fn add_assign(&mut self, other: Self) {
        *self =
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z
            };
    }
}

impl SubAssign for V3 {
    fn sub_assign(&mut self, other: Self) {
        *self =
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z
            };
    }
}

impl Sub for V3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl V3 {
    pub fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
}

impl fmt::Display for V3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "<x={0: >3}, y={1: >3}, z={2: >3}>",
            self.x, self.y, self.z,
        )
    }
}
