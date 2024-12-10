mod util;
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
    // Move the given point in this direction 1 step
    fn step(&self, point: &Point) -> Option<Point> {
        let x = point.x;
        let y = point.y;
        match self {
            Direction::South => Some(Point::new(x, y + 1)),
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    Some(Point::new(x, y - 1))
                }
            }
            Direction::East => Some(Point::new(x + 1, y)),
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    Some(Point::new(x - 1, y))
                }
            }
        }
    }
    // Turn this direction 90 degrees to the right
    fn turn(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
    fn newi32(x: i32, y: i32) -> Self {
        Point {
            x: x as usize,
            y: y as usize,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Board {
    lines: Vec<String>,
}

impl Board {
    /*
     * Visit every point in the board, call the closure f
     */
    fn walk<F>(&self, mut f: F)
    where
        F: FnMut(&Point),
    {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let p = Point { x, y };
                f(&p);
            }
        }
    }
    fn all_points(&self) -> Vec<Point> {
        let mut output = Vec::new();

        self.walk(|p| output.push(*p));

        output
    }
    fn width(&self) -> usize {
        self.lines[0].len()
    }
    fn height(&self) -> usize {
        self.lines.len()
    }
    // Is the point within the bounds of the board?
    // Points are usize so can't be negative,
    // if they wrap they will be huge and thus outside by
    // this logic anyway
    fn within(&self, point: &Point) -> bool {
        if point.y >= self.height() || point.x >= self.width() {
            false
        } else {
            true
        }
    }
    fn getu32(&self, point: &Point) -> Option<u32> {
        if let Some(val) = self.get(point) {
            match val.to_string().parse::<u32>() {
                Ok(rval) => Some(rval),
                _ => None
            }
        } else {
            None
        }
    }
    fn get(&self, point: &Point) -> Option<char> {
        let x = point.x;
        let y = point.y;

        // abort if there are no lines in the Board
        if self.lines.len() < 1 {
            return None;
        }

        // abort if y is too big
        if y >= self.lines.len() {
            return None;
        }

        // abort if x is too big (already verified y has at least 1 row)
        // this assumes all rows are the same width!
        if x >= self.lines[0].len() {
            return None;
        }

        Some(self.lines[y].chars().nth(x).expect("Must exist"))
    }
    fn set(&mut self, point: &Point, val: char) {
        if self.get(point).is_some() {
            let x = point.x;
            let y = point.y;

            let s = &mut self.lines[y];
            s.replace_range(x..x + 1, &val.to_string());
        }
    }
}

type Map = Board;

fn main() {
    println!("AoC 2024: Day 10");

    // let lines = util::get_lines_from_file("input/day10-test.txt");
    let lines = util::get_lines_from_file("input/day10.txt");

    let mut map = Map { lines };
    dbg!(&map);

    // Find the trailheads, all the 0s
    let trailheads: Vec<Point> = map
        .all_points()
        .into_iter()
        .filter(|p| map.get(p) == Some('0'))
        .collect();


    let mut total_score = 0;
    let mut total_rating = 0;

    for th in &trailheads {
        let mut ends: Vec<Point> = Vec::new();
        visit_node(&map, th, 0, &mut ends);
        // println!("TH {th:?}, Score: {}", ends.len());
        total_rating += ends.len();
        ends.sort();
        ends.dedup();
        total_score += ends.len();
    }

    println!("Part 1, total score: {}", total_score);
    println!("Part 2, total rating: {}", total_rating);
}

fn visit_node(map: &Map, pos: &Point, expected_val: u32, ends: &mut Vec<Point>) {
    if let Some(val) = map.getu32(pos) {
        if val == expected_val {
            if val == 9 {
                ends.push(*pos);
            } else {
                for direction in Direction::all() {
                    if let Some(next_pos) = direction.step(&pos) {
                        visit_node(map, &next_pos, val + 1, ends);
                    }
                }
            }
        }
    }
}
