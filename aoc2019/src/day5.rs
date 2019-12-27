use crate::common::*;
use crate::intcode::Intcode;

pub fn run() {
    let original = 
        data_file_to_string("5", "input").unwrap()
            .split(',')
            .map(|i| i.trim())
            .filter(|i| i.len() > 0)
            .map(|i| i.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
    let mut tape: Vec<isize>;
    let result: isize;

    tape = original.clone();
    let mut runner = Intcode::new(&mut tape);
    result = runner.run(5);
    println!("{}", result);
}
