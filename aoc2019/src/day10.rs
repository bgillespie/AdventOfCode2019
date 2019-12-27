use std::convert::From;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::common::*;

struct Belt {
    asteroids: Vec<(usize,usize)>,
    width    : usize,
    height   : usize,
}

impl From<&PathBuf> for Belt {
    /// "Convert" a path to a file into a Belt with all the
    /// asteroids, if it doesn't crash for some reason.
    fn from(path: &PathBuf) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut asteroids:Vec<(usize,usize)> = Vec::new();
        for (y, line) in data_lines(path).enumerate() {
            height = y;
            let mut line = match line {
                Ok(s) => s,
                Err(e) => panic!("{:?}", e.into()),
            };
            line = String::from(line.trim());
            if line.len() == 0 {
                continue;
            }
            else {
                width = std::cmp::max(width, line.len())
            }
            line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| x)
                .for_each(|x| asteroids.push((x, y)));
        }
        Belt { asteroids, width, height }
    }
}

impl Belt {
    /// Place an asteroid on a path, relative to an observation point.
    /// cx, cy - observation point; ax, ay - asteroid
    fn path_of(cx: usize, cy: usize, ax: usize, ay: usize) -> (isize, isize) {
        let mut px = ax as isize - cx as isize;
        let mut py = ay as isize - cy as isize;
        let gcd = gcd(px, py);
        px /= gcd;
        py /= gcd;
        (px, py)
    }

    /// Get closest asteroids at each angle.
    fn visible_asteroids(&self, cx:usize, cy:usize) -> HashMap<(isize, isize), usize> {
        // `visible` will become a mapping angle to index of closest asteroid
        let mut visible: HashMap<(isize, isize), usize> = HashMap::new();
        for (i, (ax, ay)) in self.asteroids.iter().enumerate() {
            // exclude central point
            if *ax == cx && *ay == cy {
                continue;
            }
            let path = Belt::path_of(cx, cy, *ax, *ay);
            if visible.contains_key(&path) {
                let j = visible[&path];
                let existing = self.asteroids[j];
                if existing.0 * existing.0 + existing.1 * existing.1 > ax * ax + ay * ay {
                    visible.insert(path, i);
                }
            }
            else {
                visible.insert(path, i);
            }
        }
        visible
    }

    fn best_station(&self) -> (usize, usize) {
        let mut best_index: usize = 0;
        let mut best_count: usize = 0;
        let mut count: usize;
        for index in 0..self.asteroids.len() {
            let (cx, cy) = self.asteroids[index];
            let count = self.visible_asteroids(cx, cy).len();
            if count > best_count {
                best_count = count;
                best_index = index;
            }
        }
        (best_index, best_count)
    }
}

#[test]
fn test() {
}

pub fn run() {
    let path = match path_to_data_file("10", "input") {
        Err(e) => panic!("{:?}", e),
        Ok(p) => p
    };
    let belt = Belt::from(&path);
    let (best_index, best_count) = belt.best_station();
    let (x, y) = belt.asteroids[best_index];
    println!("{}, {} => {}", x, y, best_count);
}