mod util;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
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

#[derive(Debug)]
struct Guard {
    pos: Point,
    start: Point,
    direction: Direction,
    history: HashSet<Point>,
}
impl Guard {
    fn new(starting_pos: Point, direction: Direction) -> Self {
        Guard {
            pos: starting_pos.clone(),
            start: starting_pos.clone(),
            direction: direction,
            history: HashSet::from([starting_pos.clone()]),
        }
    }
    fn step(&mut self, board: &Board) -> bool {
        if let Some(new_pos) = self.direction.step(&self.pos) {
            // is it allowed?
            if let Some(val) = board.get(&new_pos) {
                // if blocked, turn 90 degrees
                if val == '#' {
                    self.direction = self.direction.turn();
                    // panic!("changing direction");
                    // dbg!(&self);
                } else {
                    // If so, update the Guard (pos, history)
                    self.history.insert(new_pos.clone());
                    self.pos = new_pos;
                }
                return true;
            } else {
                // if it's gone off the board, do somethign different? return false?
                return false;
            }
        } else {
            return false;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Board {
    lines: Vec<String>,
}

impl Board {
    fn find_guard(&self) -> Guard {
        fn is_guard(p: Option<char>) -> Option<Direction> {
            if let Some(c) = p {
                match c {
                    '>' => Some(Direction::East),
                    '^' => Some(Direction::North),
                    '<' => Some(Direction::West),
                    'v' => Some(Direction::South),
                    _ => None,
                }
            } else {
                None
            }
        }

        for y in 0..self.height() {
            for x in 0..self.width() {
                let p = Point { x, y };
                if let Some(d) = is_guard(self.get(&p)) {
                    return Guard::new(p, d);
                }
            }
        }
        panic!("No guard found in this map!");
    }
    fn width(&self) -> usize {
        self.lines[0].len()
    }
    fn height(&self) -> usize {
        self.lines.len()
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

fn main() {
    println!("AoC 2024: Day 6");

    // let lines = util::get_lines_from_file("input/day6-test.txt");
    let lines = util::get_lines_from_file("input/day6.txt");

    let board = Board { lines };
    // dbg!(&board);

    let mut guard = board.find_guard();
    // dbg!(&guard);

    // Part 1
    loop {
        // dbg!(&guard);
        if !guard.step(&board) {
            break;
        }
    }

    println!("Part 1, visited squares: {}", guard.history.len());

    // Part 2
    // Using the history of the guard ONLY,
    // try putting a single extra # at each location (one at a time)
    // and then test to see if a new guard gets stuck in a loop
    // let mut b2 = board.clone();
    // let mut guard2 = b2.find_guard();
    // // dbg!(guard2);

    // // test of known place that causes a loop
    // b2.set(&Point { x: 3, y: 6}, '#');

    fn detect_loop(board: &Board, guard: &mut Guard) -> bool {
        // idea: keep track of the last len of history
        // if it hasn't grown after 3 moves, it's a loop?
        // 3 might not be enough due to types of backgracking,
        // adjust as necessary
        let mut loop_counter = 0;
        let mut last_history_len = 0;
        loop {
            loop_counter += 1;
            // dbg!(&guard);
            if !guard.step(&board) {
                break;
            }

            if loop_counter % 1000 == 0 {
                let current_history_len = guard.history.len();
                if current_history_len == last_history_len {
                    return true;
                } else {
                    last_history_len = current_history_len;
                }
            }
        }

        false // if we get here, there was no loop
    }

    let mut num_loops = 0;

    // loop over all the spots the first guard went
    for point in guard.history {
        if point == guard.start {
            continue;
        }
        let mut new_board = board.clone();
        new_board.set(&point, '#');
        let mut new_guard = new_board.find_guard();

        let is_loop = detect_loop(&new_board, &mut new_guard);
        if is_loop {
            num_loops += 1
        }
    }

    println!("Part 2, number of loops: {}", num_loops);
}
