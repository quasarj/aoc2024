mod util;

#[derive(Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn step(&self, point: &Point) -> Point {
        let x = point.x;
        let y = point.y;
        match self {
            Direction::South => Point::new(x, y + 1),
            Direction::SouthEast => Point::new(x + 1, y + 1),
            Direction::East => Point::new(x + 1, y),
            Direction::NorthEast => Point::new(x + 1, y.checked_sub(1).unwrap_or(9999)),
            Direction::North => Point::new(x, y.checked_sub(1).unwrap_or(9999)),
            Direction::NorthWest => Point::new(
                x.checked_sub(1).unwrap_or(9999),
                y.checked_sub(1).unwrap_or(9999),
            ),
            Direction::West => Point::new(x.checked_sub(1).unwrap_or(9999), y),
            Direction::SouthWest => Point::new(x.checked_sub(1).unwrap_or(9999), y + 1),
        }
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

struct Board {
    lines: Vec<String>,
}

impl Board {
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
}

fn main() {
    println!("AoC 2024: Day 4");

    // let lines = util::get_lines_from_file("input/day4-test.txt");
    let lines = util::get_lines_from_file("input/day4.txt");

    let mut total = 0;
    let mut x_total = 0;

    let board = Board { lines };

    // println!("{:?}", board.get(&Point::new(0, 0)));
    // println!("{:?}", search_x(&board, &Point::new(0, 0)));

    // look for X's to begin the XMAS
    for y in 0..board.height() {
        for x in 0..board.width() {
            let point = Point::new(x, y);
            if let Some(val) = board.get(&point) {
                match val {
                    'X' => {
                        let count = search(&board, &point);
                        total += count;
                    }
                    'A' => {
                        // time for the X-MAS
                        let count = search_x(&board, &point);
                        x_total += count;
                    }
                    _ => {}
                }
            }
        }
        // println!("");
    }

    println!("Part 1: {}", total);
    println!("Part 2: {}", x_total);
}

fn search_x(board: &Board, point: &Point) -> usize {
    // M.S
    // .A.
    // M.S
    if l_in_d(board, point, Direction::NorthWest, 'M')
        && l_in_d(board, point, Direction::NorthEast, 'S')
        && l_in_d(board, point, Direction::SouthWest, 'M')
        && l_in_d(board, point, Direction::SouthEast, 'S')
    {
        // println!("type a");
        return 1;
    }

    // M.M
    // .A.
    // S.S
    if l_in_d(board, point, Direction::NorthWest, 'M')
        && l_in_d(board, point, Direction::NorthEast, 'M')
        && l_in_d(board, point, Direction::SouthWest, 'S')
        && l_in_d(board, point, Direction::SouthEast, 'S')
    {
        // println!("type b");
        return 1;
    }

    // S.M
    // .A.
    // S.M
    if l_in_d(board, point, Direction::NorthWest, 'S')
        && l_in_d(board, point, Direction::NorthEast, 'M')
        && l_in_d(board, point, Direction::SouthWest, 'S')
        && l_in_d(board, point, Direction::SouthEast, 'M')
    {
        // println!("type c");
        return 1;
    }

    // S.S
    // .A.
    // M.M
    if l_in_d(board, point, Direction::NorthWest, 'S')
        && l_in_d(board, point, Direction::NorthEast, 'S')
        && l_in_d(board, point, Direction::SouthWest, 'M')
        && l_in_d(board, point, Direction::SouthEast, 'M')
    {
        // println!("type d");
        return 1;
    }

    0
}

fn l_in_d(board: &Board, point: &Point, direction: Direction, letter: char) -> bool {
    let next = direction.step(point);

    if let Some(l) = board.get(&next) {
        if l == letter {
            true
        } else {
            false
        }
    } else {
        false
    }
}

// Search for XMAS in all directions, return count found
fn search(board: &Board, point: &Point) -> usize {
    search_direction(board, point, Direction::North)
        + search_direction(board, point, Direction::South)
        + search_direction(board, point, Direction::East)
        + search_direction(board, point, Direction::West)
        + search_direction(board, point, Direction::NorthEast)
        + search_direction(board, point, Direction::NorthWest)
        + search_direction(board, point, Direction::SouthEast)
        + search_direction(board, point, Direction::SouthWest)
}

// search in one direction
fn search_direction(board: &Board, point: &Point, direction: Direction) -> usize {
    let next = direction.step(point);

    if let Some(_l @ 'M') = board.get(&next) {
        let next = direction.step(&next); // next the next next
        if let Some(_l @ 'A') = board.get(&next) {
            let next = direction.step(&next); // next the next next next
            if let Some(_l @ 'S') = board.get(&next) {
                return 1;
            }
        }
    }

    0
}
