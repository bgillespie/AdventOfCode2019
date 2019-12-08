use crate::common::*;

fn add(a: usize, b: usize) -> usize { a + b }
fn mul(a: usize, b: usize) -> usize { a * b }

const OPERATOR:[fn(usize, usize) -> usize;2] = [add, mul];
const TARGET: usize = 19690720;

struct Intcode<'a> {
    tape: &'a mut Vec<usize>,
    pointer: usize,
}

impl<'a> Intcode<'a> {
    fn new(tape: &'a mut Vec<usize>) -> Self {
        Intcode { tape, pointer: 0 }
    }

    fn run(&'a mut self) -> usize {
        loop{
            let opcode = self.tape[self.pointer];
            match opcode {
                1 | 2 => {
                    let operand_pos_1 = self.tape[self.pointer + 1];
                    let operand_pos_2 = self.tape[self.pointer + 2];
                    let result_pos = self.tape[self.pointer + 3];
                    let value =
                        OPERATOR[opcode - 1](
                            self.tape[operand_pos_1],
                            self.tape[operand_pos_2]
                        );
                    self.tape[result_pos] = value;
                    // println!("Set position {} to {}", result_pos, value);
                    self.pointer += 4;
                    // println!("Moved to position {}", self.pointer);
                },
                99 => {
                    // println!("Finished!");
                    break;
                },
                _ => {self.tape[0] = 0; break;}
            }
        }
        self.tape[0]
    }
}


pub fn run() {
    let original = 
        file_to_string("day/2/input")
            .split(',')
            .map(|i| i.trim())
            .filter(|i| i.len() > 0)
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
    let mut tape: Vec<usize>;
    let mut result: usize;

    // We don't know if the program is self-modifying
    // so just brute-force it!
    'search: for noun in 0..100 {
        for verb in 0..100 {
            tape = original.clone();
            tape[1] = noun;
            tape[2] = verb;
            let mut runner = Intcode::new(&mut tape);
            result = runner.run();
            if result == TARGET {
                println!("{}", 100 * noun + verb);
                break 'search;
            }
        }
    }
}