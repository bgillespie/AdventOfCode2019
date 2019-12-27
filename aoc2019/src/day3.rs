use std::cmp::{min,max};

use crate::common::*;

type Coords = (isize, isize);

#[derive(PartialEq,Debug,Clone)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(PartialEq,Debug,Clone)]
struct Line {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
    orientation: Orientation,
}

// Didn't need to do all this... oh well :-/

impl Line {
    fn new(x1: isize, y1: isize, x2: isize, y2: isize) -> Line {
        let orientation: Orientation;
        if x1 == x2 {
            orientation = Orientation::Vertical;
        }
        else if y1 == y2 {
            orientation = Orientation::Horizontal;
        }
        else {
            panic!("Wonky line: ({}, {}) -> ({}, {})", x1, y1, x2, y2);
        }
        Line { x1, y1, x2, y2, orientation }
    }

    /// Does the first value fall between the other two?
    #[inline]
    fn between(mid: isize, a: isize, b: isize) -> bool {
        (a <= mid && b >= mid) || (b <= mid && a >= mid)
    }

    /// Get the intercept point of this line and another
    /// (if any)
    fn intercept_point(&self, other: &Line) -> Option<Coords> {
        if self.orientation == other.orientation {
            // Lines are parallel
            None
        }
        else {
            let (horiz, vert) =
                if self.orientation == Orientation::Vertical {
                    (other, self)
                }
                else {
                    (self, other)
                };
            if Line::between(vert.x1, horiz.x1, horiz.x2)
            && Line::between(horiz.y1, vert.y1, vert.y2) {
                Some((vert.x1, horiz.y1))
            }
            else {
                None
            }
        }
    }

    /// Get the number of steps in the line.
    fn blocks(&self) -> usize {
        ((max(self.x1, self.x2) - min(self.x1,self.x2)) +
         (max(self.y1, self.y2) - min(self.y1,self.y2)))
        as usize
    }
}


#[test]
fn test_len() {
    let tests = [
        (1,0,1,0,0),
        (1,0,2,0,1),
        (2,0,1,0,1),
        (0,0,-1,0,1),
        (1,0,1,-1,1),
    ];
    for t in tests.iter() {
        let line = Line::new(t.0,t.1,t.2,t.3);
        assert_eq!(line.blocks(), t.4);
    }
}


/// Convert string to coordinate change
fn spec_to_vec(spec: &str) -> Coords {
    let (direction, value) = (&spec[0..1], &spec[1..]);
    let value = value.parse::<isize>().unwrap();
    match direction {
        "L" => (-value, 0),
        "R" => (value, 0),
        "U" => (0, -value),
        "D" => (0, value),
        _ => panic!("weird input \"{}\"", spec)
    }
}


/// Convert a row string to moves
fn row_moves(source: &str) -> Vec<Coords> {
    let rows_of_moves:Vec<Coords> = 
        source
            .split(",")              // break row on commas
            .map(|i| spec_to_vec(i)) // convert coordinate deltas
            .collect();
    rows_of_moves
}


/// Load rows of moves
fn rows_of_moves(source: String) -> Vec<Vec<Coords>> {
    let rows_of_moves:Vec<Vec<Coords>> = 
        source
            .split("\n")                  // split into rows
            .map(|row| row.trim())        // trim excess whitespace
            .filter(|row| row.len() > 0)  // exclude empty lines
            .map(|row| row_moves(row))
            .collect();
    rows_of_moves
}


/// Turn moves into lines
fn moves_to_lines(moves: &Vec<Coords>) -> Vec<Line> {
    moves
        .iter()
        .scan( (0, 0, 0, 0), |curr, (x, y)| {
            // join points into lines with
            // (old_x, old_y, new_x, new_y)
            *curr = (curr.2, curr.3, curr.2 + x, curr.3 + y);
            Some(Line::new(curr.0, curr.1, curr.2, curr.3))
        })
    .collect()
}


/// Turn rows of moves to rows of lines
fn rows_of_lines(rows_of_moves: &Vec<Vec<Coords>>) -> Vec<Vec<Line>> {
    rows_of_moves
        .iter()
        .map(|row| moves_to_lines(row))
        .collect()
}


/// Get the crossing points from a bunch of lines
/// Gets the coords and the number of lines taken to the crossing on both paths.
fn crossings(path_a: &Vec<Line>, path_b: &Vec<Line>) -> Vec<(Coords, usize, usize)> {
    let mut crossings: Vec<(Coords, usize, usize)> = Vec::new();
    for (i_a, line_a) in path_a.iter().enumerate() {
        for (i_b, line_b) in path_b.iter().enumerate() {
            if let Some(intercept) = line_a.intercept_point(line_b) {
                crossings.push((intercept, i_a, i_b));
                // println!("({}, {}):{}, {}", intercept.0, intercept.1, i_a, i_b);
            }
        }
    }
    crossings
}


fn steps_in(lines: &[Line]) -> usize {
    lines
        .iter()
        .map(|i| i.blocks())
        .sum::<usize>()
}


#[test]
fn test_steps_in() {
    let tests = [
        ("R1", 1),
        ("R1,U1", 2),
        ("R1,U1,L1", 3),
        ("R1,U1,L1,D1", 4),
        ("R1,U1,R1,U2", 5),
    ];
    for test in tests.iter() {
        let lines = moves_to_lines(&row_moves(test.0));
        print!("{} ", lines.iter().map(|i| format!("{:?}", i)).collect::<Vec<String>>().join(", "));
        let actual = steps_in(&lines);
        // println!("   {} <=> {}", actual, test.1);
        assert_eq!(actual, test.1);
    }
}


pub fn part1() {
    let source = data_file_to_string("3", "input").unwrap();
    let moves = rows_of_moves(source);
    let lines = rows_of_lines(&moves);
    let crossings = crossings(&lines[0], &lines[1]);
    let mut answer = &crossings[1].0;
    for (crossing, _, _) in crossings.iter().skip(1) {
        println!("{:?}", crossing);
        if (crossing.0.abs() + crossing.1.abs()) < (answer.0.abs() + answer.1.abs()) {
            answer = &crossing;
        }
    }
    println!("{}", answer.0.abs() + answer.1.abs());
}


pub fn part2() {
    let source = data_file_to_string("3", "input").unwrap();
    let moves = rows_of_moves(source);
    let lines = rows_of_lines(&moves);
    let crossings = crossings(&lines[0], &lines[1]);
    let mut answer = 1_000_000;
    for (crossing, i_a, i_b) in crossings.iter() {
        let mut dist = steps_in(&lines[0][..*i_a]) + steps_in(&lines[1][..*i_b]);
        // account for final coord up till intersection point
        if *i_a > 0 {
            dist += Line::new(lines[0][*i_a - 1].x2, lines[0][*i_a - 1].y2, crossing.0, crossing.1).blocks();
        }
        else {
            dist += Line::new(0, 0, crossing.0, crossing.1).blocks();
        }
        if *i_b > 0 {
            dist += Line::new(lines[1][*i_b - 1].x2, lines[1][*i_b - 1].y2, crossing.0, crossing.1).blocks();
        }
        else {
            dist += Line::new(0, 0, crossing.0, crossing.1).blocks();
        }
        println!("{:?} @ {}, {} dist {}", crossing, *i_a, *i_b, dist);
        answer = min(dist, answer);
    }
    println!("{}", answer);
}
