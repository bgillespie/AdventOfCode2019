use std::fmt;
use std::collections::HashSet;

use lazy_static::*;
use regex::Regex;

use crate::common::*;

lazy_static! { 
    static ref RE_INPUT: Regex = Regex::new(r"(?x)
        ^<\s*x\s*=\s*(-?\d+)\s*,
        \s*y\s*=\s*(-?\d+)\s*,
        \s*z\s*=\s*(-?\d+)\s*
        \s*>$").unwrap();
}
      
//
// Moons
//

#[derive(PartialEq,Eq,Hash,Clone)]
struct Moon {
    pos: V3,
    vel: V3,
}

impl Moon {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Moon {
            pos: V3 {x, y, z},
            vel: V3 {x: 0, y: 0, z: 0}
        }
    }

    fn apply_force(&mut self, force: V3) {
        self.pos += force;
    }

    fn apply_velocity(&mut self) {
        self.apply_force(self.vel);
    }

    fn alter_velocity(&mut self, change: &V3) {
        self.vel += *change;
    }

    fn total_energy(&self) -> usize {
        let pot = self.pos.abs();
        let kin = self.vel.abs();
        let tot_pot = pot.x + pot.y + pot.z;
        let tot_kin = kin.x + kin.y + kin.z;
        (tot_pot * tot_kin) as usize
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "pos={}, vel={}",
            self.pos, self.vel,
        )
    }
}

//
// Jupiter
//

#[derive(PartialEq,Eq,Hash,Clone)]
struct Jupiter {
    moons: Vec<Moon>,
}

impl fmt::Display for Jupiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\nTOTAL ENERGY {}\n",
            self.moons
                .iter()
                .map(|i| format!("{} energy {}",
                                 i, i.total_energy())
                    )
                .collect::<Vec<String>>()
                .join("\n"),
            self.moons
                .iter()
                .map(|i| i.total_energy())
                .sum::<usize>()            
        )
    }
}

impl From<&std::path::PathBuf> for Jupiter {
    fn from(p: &std::path::PathBuf) -> Self {
        let mut moons:Vec<Moon> = Vec::new();
        for line in data_lines(p) {
            let mut line = match line {
                Ok(line) => line,
                Err(e) => panic!("{:?}", e.into()),
            };
            line = String::from(line.trim());
            for cap in RE_INPUT.captures_iter(&line) {
                moons.push(
                    Moon::new(
                        cap[1].parse::<isize>().unwrap(),
                        cap[2].parse::<isize>().unwrap(),
                        cap[3].parse::<isize>().unwrap(),
                    )
                )
            }
        }
        moons.shrink_to_fit();
        Jupiter { moons }
    }
}

impl Jupiter {
    fn apply_gravity(&mut self) {
        let mut changes: Vec<V3> =
            vec![V3{x:0, y:0, z:0};self.moons.len()];
        for (i, j) in TwoPermute::new(self.moons.len() - 1) {
            changes[j] += (self.moons[i].pos - self.moons[j].pos).signum();
            changes[i] += (self.moons[j].pos - self.moons[i].pos).signum();            
        }
        for (force, moon) in changes.iter().zip(self.moons.iter_mut()) {
            moon.alter_velocity(force);
        }
    }

    fn apply_velocity(&mut self) {
        for moon in &mut self.moons {
            moon.apply_velocity();
        }
    }

    fn step(&mut self) {
        self.apply_gravity();
        self.apply_velocity();
    }
}

pub fn run() {
    // load the system
    let path = match path_to_data_file("11", "input") {
        Err(e) => panic!("{:?}", e),
        Ok(p) => p
    };
    let mut history: HashSet<Jupiter> = HashSet::new();
    let mut jovian = Jupiter::from(&path);
    println!("{}", jovian);
    history.insert(jovian.clone());

    let mut step = 0;
    loop {
        jovian.step();
        step += 1;
        if history.contains(&jovian) {
            break;
        }
        history.insert(jovian.clone());
        if step % 100000 == 0 {
            println!("Step {}", step);
            println!("{}", jovian);
        }
    }
    println!("Took {} steps to repeat", step);
}

