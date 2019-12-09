pub struct Intcode<'a> {
    tape: &'a mut Vec<isize>,
    pointer: usize,
}

impl<'a> Intcode<'a> {
    pub fn new(tape: &'a mut Vec<isize>) -> Self {
        Intcode { tape, pointer: 0 }
    }

    fn read(&mut self, mode: isize) -> isize {
        let result = 
            match mode % 10 {
                0 => {
                    self.tape[self.tape[self.pointer] as usize]
                },
                1 => {
                    self.tape[self.pointer]
                },
                _ => panic!("Unknown mode {}", mode),
            };
        self.pointer += 1;
        result
    }

    pub fn run(&'a mut self, mut input: isize) -> isize {
        loop {
            let opcode = self.read(1);
            let mode = opcode / 100;
            let opcode = opcode % 100;

            // EXIT code
            if opcode == 99 { break; }
            
            match opcode {
                1 => {  // ADD
                    let operand_1 = self.read(mode);
                    let operand_2 = self.read(mode / 10);
                    let result_pos = self.read(1);
                    let value = operand_1 + operand_2;
                    self.tape[result_pos as usize] = value;
                },
                2 => {  // MUL
                    let operand_1 = self.read(mode);
                    let operand_2 = self.read(mode / 10);
                    let result_pos = self.read(1);
                    let value = operand_1 * operand_2;
                    self.tape[result_pos as usize] = value;
                },
                3 => {  // STORE
                    let result_pos = self.read(1);
                    self.tape[result_pos as usize] = input;
                },
                4 => { // RETRIEVE
                    input = self.read(mode);
                },
                5 => {  // JNZ
                    let operand_1 = self.read(mode);
                    let result_pos = self.read(mode / 10);
                    if operand_1 != 0 {
                        self.pointer = result_pos as usize;
                    }
                },
                6 => {  // JZ
                    let operand_1 = self.read(mode);
                    let result_pos = self.read(mode / 10);
                    if operand_1 == 0 {
                        self.pointer = result_pos as usize;
                    }
                },
                7 => {  // LT
                    let operand_1 = self.read(mode);
                    let operand_2 = self.read(mode / 10);
                    let result_pos = self.read(1);
                    let value = if operand_1 < operand_2 { 1 } else { 0 };
                    self.tape[result_pos as usize] = value;
                },
                8 => {  // EQ
                    let operand_1 = self.read(mode);
                    let operand_2 = self.read(mode / 10);
                    let result_pos = self.read(1);
                    let value = if operand_1 == operand_2 { 1 } else { 0 };
                    self.tape[result_pos as usize] = value;
                },
                99 => {
                    break;
                },
                _ => {
                    panic!("Unknown opcode {}", opcode);
                }
            }
        }
        input
    }
}

#[test]
fn test_run() {
    let example: Vec<isize> = vec![3,9,8,9,10,9,4,9,99,-1,8];
    let tests = [(7, 0), (8, 1), (9, 0), (256, 0)];
    println!("Equal to eight?");
    for (test, expected) in tests.iter() {
        let mut tape = example.clone();
        let mut ic = Intcode::new(&mut tape);
        let actual = ic.run(*test);
        assert_eq!(*expected, actual);
    }

    let example: Vec<isize> = vec![3,3,1108,-1,8,3,4,3,99];
    let tests = [(7, 0), (8, 1), (9, 0), (256, 0)];
    println!("Equal to eight?");
    for (test, expected) in tests.iter() {
        let mut tape = example.clone();
        let mut ic = Intcode::new(&mut tape);
        let actual = ic.run(*test);
        assert_eq!(*expected, actual);
    }

    let example: Vec<isize> = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    let tests = [(0, 0), (8, 1), (9, 1), (-256, 1)];
    println!("Zero or non-zero?");
    for (test, expected) in tests.iter() {
        let mut tape = example.clone();
        let mut ic = Intcode::new(&mut tape);
        let actual = ic.run(*test);
        assert_eq!(*expected, actual);
    }

    let example:Vec<isize> = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    let tests = [(7, 999), (8, 1000), (9, 1001), (256, 1001), (-256, 999)];
    println!("Relative to eight?");
    for (test, expected) in tests.iter() {
        let mut tape = example.clone();
        let mut ic = Intcode::new(&mut tape);
        let actual = ic.run(*test);
        assert_eq!(*expected, actual);
    }
}

