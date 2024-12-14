#![allow(dead_code)]
use std::collections::HashMap;
use std::fmt;
use std::fs::read_to_string;

pub fn get_lines_from_file(filename: &str) -> Vec<String> {
    let input_file = read_to_string(filename).unwrap();

    let lines = input_file.lines().map(|line| line.to_string()).collect();

    lines
}

pub fn count_frequencies(numbers: &[i32]) -> HashMap<i32, usize> {
    let mut frequency_map = HashMap::new();

    for &num in numbers {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    frequency_map
}

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
    // Move the given point in this direction 1 step
    // this version always steps even if it goes negative
    pub fn step(&self, point: &Point) -> Point {
        let x = point.x;
        let y = point.y;
        match self {
            Direction::South => Point::new(x, y + 1),
            Direction::North => Point::new(x, y - 1),
            Direction::East => Point::new(x + 1, y),
            Direction::West => Point::new(x - 1, y),
        }
    }
    // Turn this direction 90 degrees to the right
    pub fn turn(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

type PointInt = i32;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
pub struct Point {
    pub x: PointInt,
    pub y: PointInt,
}

impl Point {
    pub fn new(x: PointInt, y: PointInt) -> Self {
        Point { x, y }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub lines: Vec<String>,
}

impl Board {
    /*
     * Visit every point in the board, call the closure f
     */
    pub fn walk<F>(&self, mut f: F)
    where
        F: FnMut(&Point),
    {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let p = Point {
                    x: x as PointInt,
                    y: y as PointInt,
                };
                f(&p);
            }
        }
    }
    pub fn all_points(&self) -> Vec<Point> {
        let mut output = Vec::new();

        self.walk(|p| output.push(*p));

        output
    }
    pub fn width(&self) -> usize {
        self.lines[0].len()
    }
    pub fn height(&self) -> usize {
        self.lines.len()
    }
    // Is the point within the bounds of the board?
    // Points are usize so can't be negative,
    // if they wrap they will be huge and thus outside by
    // this logic anyway
    pub fn within(&self, point: &Point) -> bool {
        if point.y as usize >= self.height() || point.x as usize >= self.width() {
            false
        } else {
            true
        }
    }
    pub fn getu32(&self, point: &Point) -> Option<u32> {
        if let Some(val) = self.get(point) {
            match val.to_string().parse::<u32>() {
                Ok(rval) => Some(rval),
                _ => None,
            }
        } else {
            None
        }
    }
    pub fn get(&self, point: &Point) -> Option<char> {
        let x = point.x;
        let y = point.y;

        // abort if there are no lines in the Board
        if self.lines.len() < 1 {
            return None;
        }

        // abort if y is too big
        if y as usize >= self.lines.len() {
            return None;
        }

        // abort if x is too big (already verified y has at least 1 row)
        // this assumes all rows are the same width!
        if x as usize >= self.lines[0].len() {
            return None;
        }

        Some(
            self.lines[y as usize]
                .chars()
                .nth(x as usize)
                .expect("Must exist"),
        )
    }
    pub fn set(&mut self, point: &Point, val: char) {
        if self.get(point).is_some() {
            let x = point.x as usize;
            let y = point.y as usize;

            let s = &mut self.lines[y];
            s.replace_range(x..x + 1, &val.to_string());
        }
    }
}
