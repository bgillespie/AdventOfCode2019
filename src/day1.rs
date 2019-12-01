use crate::common::*;

fn calc_fuel(mass: isize) -> isize {
    let fuel = mass as isize / 3 - 2;
    if fuel < 1 {
        0
    }
    else {
        fuel + calc_fuel(fuel)
    }
}

pub fn run() {
    let source = file_to_string("day/1/input");
    let lines = 
        source
            .split("\n")
            .map(|line| line.trim())
            .filter(|line| line.len() > 0);
    let result: usize = 
        lines
            .map(|m| m.parse::<usize>().unwrap())
            //.map(|m| m / 3 - 2)
            .map(|m| calc_fuel(m as isize) as usize)
            .sum();
    println!("{}", result);
}
