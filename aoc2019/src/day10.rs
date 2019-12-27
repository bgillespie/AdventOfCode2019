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

    /// Get asteroids at each angle.
    /// Return mapping of angle to vec of asteroid indices, in order of closest first.
    fn visible_asteroids(&self, cx:usize, cy:usize) -> HashMap<usize, Vec<usize>> {

        // First get mapping of angle to vec of asteroids at that angle
        let mut visible: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, (ax, ay)) in self.asteroids.iter().enumerate() {
            // exclude central point
            if *ax == cx && *ay == cy {
                continue;
            }
            // determine angle to use as key
            let angle = (
                f32::atan2(
                    (cy as isize - *ay as isize) as f32,
                    (cx as isize - *ax as isize) as f32
                ) * F32_RAD_TO_DEG
            ) as usize;
            visible.entry(angle).or_insert(Vec::new()).push(i);
        }

        // Now sort all vecs of asteroids by closeness to focal point
        for v in visible.values_mut() {
            v.sort_by(
                |i, j| {
                    let (ix, iy) = self.asteroids[*i];
                    let (jx, jy) = self.asteroids[*j];
                    let i_size = (cx as isize - ix as isize).pow(2) + (cy as isize - iy as isize).pow(2);
                    let j_size = (cx as isize - jx as isize).pow(2) + (cy as isize - jy as isize).pow(2);
                    if i_size < j_size {
                        std::cmp::Ordering::Less
                    }
                    else if i_size > j_size {
                        std::cmp::Ordering::Greater
                    }
                    else {
                        std::cmp::Ordering::Equal
                    }
                }
            );
        }
        visible
    }

    fn best_station(&self) -> (usize, HashMap<usize, Vec<usize>>) {
        let mut best_index: usize = 0;
        let mut best_count: usize = 0;
        let mut best_visibles = HashMap::new();
        let mut visibles;
        let mut count: usize;
        for index in 0..self.asteroids.len() {
            let (cx, cy) = self.asteroids[index];
            visibles = self.visible_asteroids(cx, cy);
            count = visibles.len();
            if count > best_count {
                best_count = count;
                best_index = index;
                best_visibles = visibles;
            }
        }
        (best_index, best_visibles)
    }
}

#[test]
fn test() {
}

pub fn run() {
    let path = match path_to_data_file("10", "example_4") {
        Err(e) => panic!("{:?}", e),
        Ok(p) => p
    };
    let belt = Belt::from(&path);
    let (best_index, best_visibles) = belt.best_station();
    let (x, y) = belt.asteroids[best_index];
    println!("{}, {} => {}", x, y, best_visibles.len());
    
    let mut best_visibles = best_visibles;
    let mut count = 0;
    'laser: loop {
        let mut angles:Vec<usize> = best_visibles.keys().map(|i| *i).collect();
        angles.sort();
        for angle in angles {
            if best_visibles[&angle].len() > 0 {
                count += 1;
                let (x, y) = belt.asteroids[best_visibles[&angle][0]];
                best_visibles.entry(angle).and_modify(|v| {v.remove(0);});
                println!("{} => ({}, {})", count, x, y);
                if count == 200 {
                    break 'laser
                }
            }
        }
    }
}