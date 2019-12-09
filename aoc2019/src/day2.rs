use crate::common::*;
use crate::intcode::Intcode;

const TARGET: isize = 19690720;

pub fn run() {
    let original = 
        file_to_string("day/2/input")
            .split(',')
            .map(|i| i.trim())
            .filter(|i| i.len() > 0)
            .map(|i| i.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
    let mut tape: Vec<isize>;
    let mut result: isize;

    // We don't know if the program is self-modifying
    // so just brute-force it!
    'search: for noun in 0..100 {
        for verb in 0..100 {
            tape = original.clone();
            tape[1] = noun;
            tape[2] = verb;
            let mut runner = Intcode::new(&mut tape);
            runner.run(0);
            result = tape[0];
            if result == TARGET {
                println!("{}", 100 * noun + verb);
                break 'search;
            }
        }
    }
    println!("Finished");
}