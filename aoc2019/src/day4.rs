use std::iter::Iterator;

const FIRST:usize = 235741;
const LAST:usize = 706948;

struct PwSearch {
    last   : usize,
    current: usize,
}

impl PwSearch {
    fn new(first: usize, last: usize) -> Self {
        PwSearch {last, current: first, }
    }

    fn digits(num: usize) -> Vec<u8> {
        num
            .to_string()
            .chars()
            .map(|i| i.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
    }

    fn check_digits(digits: Vec<u8>) -> bool {
        let mut two_adj = false;
        let mut count_adj = 1u8;
        let mut prev = digits[0];
        for digit in digits.into_iter().skip(1) {
            if digit == prev {
                count_adj += 1;
            }
            else {
                two_adj = two_adj || count_adj == 2;
                count_adj = 1;
            }
            if prev > digit {
                return false;
            }
            prev = digit;
        }
        two_adj || count_adj == 2
    }

    fn check_usize(num: usize) -> bool {
        Self::check_digits(Self::digits(num))
    }
}

#[test]
fn test_check_usize() {
    let tests = [
        (111111, false),
        (112233, true),
        (123444, false),
        (123789, false),
        (111122, true),
    ];
    for test in &tests {
        let rez = PwSearch::check_usize(test.0);
        println!("{:?} => {:?}", test, rez);
        assert_eq!(rez, test.1);
    }
}

impl Iterator for PwSearch {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        'search: for num in self.current..=self.last {
            if Self::check_usize(num) {
                self.current = num + 1;
                return Some(num);
            }
        }
        None
    }
}

pub fn part1() {
    // Could filter the first and last values before searching,
    // but ain't nobody got time for that
    let p = PwSearch::new(FIRST, LAST).count();
    println!("{}", p);
}
