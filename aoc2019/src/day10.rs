use std::convert::From;
use std::path::PathBuf;
use std::collections::BTreeMap;
use std::cmp::PartialEq;
use crate::common::*;

// Angle multiplier for accuracy
const ANGLE_MULT: f32 = 100.0;
// Angle*ANGLE_MULT (from upright) and square of distance
type Dist2 = usize;
type AngleM = usize;
type Polar = (AngleM, Dist2);
type AsteroidIdx = usize;

/*
 * ASTEROID
 */

struct Asteroid {
    x: usize,
    y: usize,
    lines_of_sight: BTreeMap<AngleM, BTreeMap<Dist2,AsteroidIdx>>,
}

/// If asteroids are at the exact same position, then they're the
/// same asteroid, for the purposes of this
impl PartialEq for Asteroid {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Asteroid {
    fn new(x: usize, y: usize) -> Asteroid {
        Asteroid { x, y, lines_of_sight: BTreeMap::new() }
    }

    /// Calculate Polar of an asteroid relative to this one.
    fn polar(&self, other: &Asteroid) -> Polar {
        let px = other.x as isize - self.x as isize;
        let py = other.y as isize - self.y as isize;
        let dist2 = (px.pow(2) + py.pow(2)) as Dist2;
        let anglem = f32::atan2(py as f32, px as f32)
                    * 180f32 / std::f32::consts::PI;
        let anglem = (((anglem + 450f32) % 360f32) * ANGLE_MULT) as AngleM;
        (anglem, dist2)
    }
}


/*
 * BELT
 */

struct Belt {
    asteroids: Vec<Asteroid>,
}

impl From<&PathBuf> for Belt {
    /// "Convert" a file-path into a Belt with all the
    /// Asteroids, if it doesn't crash for some I/O reason.
    fn from(path: &PathBuf) -> Self {
        // Read lines from the input and convert them to new Asteroids
        let mut asteroids:Vec<Asteroid> = Vec::new();
        for (y, line) in data_lines(path).enumerate() {
            let mut line = match line {
                Ok(s) => s,
                Err(e) => panic!("{:?}", e.into()),
            };
            line = String::from(line.trim());
            if line.len() == 0 {
                continue;
            }
            line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| x)
                .for_each(|x| asteroids.push(Asteroid::new(x, y)));
        }

        // Initialize all the lines of sight for every asteroid
        let mut polar;
        for i_pov in 0..asteroids.len() {
            polar = Belt::calc_lines_of_sight(i_pov, &asteroids);
            asteroids[i_pov].lines_of_sight = polar;
        }
        Belt { asteroids }
    }
}


impl Belt {

    /// For every line of sight, map distance to asteroid (index)
    fn calc_lines_of_sight(i_pov: AsteroidIdx, asteroids: &Vec<Asteroid>) 
    -> BTreeMap<AngleM, BTreeMap<Dist2,AsteroidIdx>> {
        let pov = &asteroids[i_pov];
        let mut polars: BTreeMap<AngleM,BTreeMap<Dist2,AsteroidIdx>> = 
            BTreeMap::new();
        let mut polar;
        for (i, asteroid) in asteroids.iter().enumerate() {
            if i != i_pov {  // ignore pov asteroid
                polar = pov.polar(asteroid);
                polars
                    .entry(polar.0)
                    .or_insert(BTreeMap::new())
                    .insert(polar.1, i);
            }
        }
        polars
    }

    /// Find the asteroid with best visibility of other asteroids.
    /// Whichever has most lines of sight wins.
    fn best_visibility(&self) -> &Asteroid {
        self.asteroids
            .iter()
            .fold(&self.asteroids[0],
                  |acc, i|
                  if i.lines_of_sight.len() > acc.lines_of_sight.len() {
                      i
                  }
                  else {
                      acc
                  }
                )
    }

}


pub fn run() {
    // load the asteroid field
    let path = match path_to_data_file("10", "input") {
        Err(e) => panic!("{:?}", e),
        Ok(p) => p
    };
    let belt = Belt::from(&path);

    // Show how many asteroids and the one that can see the most
    println!("{}", belt.asteroids.len());
    let pov = belt.best_visibility();
    println!("{}, {} => {}", pov.x, pov.y, pov.lines_of_sight.len());

    // iterate over iterators in each line of sight
    // all ordered by key bcoz btreemaps
    let mut los_iter = pov.lines_of_sight
        .values()  // values: Dist2 => AsteroidIdx 
        .map(|di|
             di.values())  // each is now AsteroidIdx
        .collect::<Vec<_>>();
    let mut count = 0;
    let mut andidx: usize = 0;
    'layzur: loop {
        for ang in &mut los_iter {
            if let Some(a) = ang.next() {
                andidx = *a;
                count += 1;
                if count == 200 {
                    break 'layzur;
                }
            }
        }
    }
    let lasteroid = &belt.asteroids[andidx];
    println!("RAH!!! {}, {}", lasteroid.x, lasteroid.y);
}
