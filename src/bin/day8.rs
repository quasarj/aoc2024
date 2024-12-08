mod util;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
    fn newi32(x: i32, y: i32) -> Self {
        Point { x: x as usize, y: y as usize }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Board {
    lines: Vec<String>,
}

impl Board {
    fn all_symbols(&self) -> HashMap<char, Vec<Point>> {
        let mut map = HashMap::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let p = Point { x, y };
                if let Some(c) = self.get(&p) {
                    if c != '.' {
                        map.entry(c).or_insert(Vec::new()).push(p);
                    }
                }
            }
        }

        map
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
        if point.y >= self.height() ||
            point.x >= self.width() 
        {
            false
        } else {
            true
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

fn main() {
    println!("AoC 2024: Day 8");

    // let lines = util::get_lines_from_file("input/day8-test.txt");
    // let lines = util::get_lines_from_file("input/day8-test1.txt");
    let lines = util::get_lines_from_file("input/day8.txt");
    
    let board = Board { lines };

    // idea:
    // 1: make a list of all symbols
    // 2: build HashMap<Symbol, Vec<Point>> of them ??
    // might not need 1 or 2, see 3
    // 3(2): for each symbol in the map:
    //          for each pair (zip?) of Points:
    //          calculate the dX, and dY
    //          from the left point, add dX,dY
    //          from the right point, subtract dX,dY
    //          this might work
    dbg!(&board);

    let symbols = board.all_symbols();
    
    let mut all_antinodes: Vec<Point> = Vec::new();

    for (key, a) in symbols.iter() {
        for p in a {
            for pp in a {
                // println!("{:?} -> {:?}", p, pp);
                if p != pp {
                    let mut ans = antinodes(&board, &p, &pp);
                    all_antinodes.append(&mut ans);
                }
            }
        }
    }

    let mut test: HashSet<Point> = HashSet::new();
    for v in all_antinodes {
        test.insert(v);
    }

    println!("{}", test.len());
    // dbg!(test);

    // let p1 = Point::new(4, 3);
    // let p2 = Point::new(8, 4);
    // dbg!(antinodes(&board, &p1, &p2));
}

fn antinodes(board: &Board, p1: &Point, p2: &Point) -> Vec<Point> {
    let mut antinodes: Vec<Point> = Vec::new();

    // calculate dX
    let dX = p2.x as i32 - p1.x as i32;
    let dY = p2.y as i32 - p1.y as i32;

    // dbg!(dX, dY);

    // generate all the new points from both ends
    // continue + until off the board
    let mut x = p1.x as i32;
    let mut y = p1.y as i32;
    loop {
        x += dX;
        y += dY;
        let new_point = Point::newi32(x, y);
        if !board.within(&new_point) {
            break;
        }
        antinodes.push(new_point);
    }

    // continue - until off the board
    let mut x = p1.x as i32;
    let mut y = p1.y as i32;
    loop {
        x -= dX;
        y -= dY;
        let new_point = Point::newi32(x, y);
        if !board.within(&new_point) {
            break;
        }
        antinodes.push(new_point);
    }

    // antinodes.push(Point::newi32(p1.x as i32 - dX, p1.y as i32 - dY));

    // antinodes.push(Point::newi32(p2.x as i32 + dX, p2.y as i32 + dY));
    // antinodes.push(Point::newi32(p2.x as i32 - dX, p2.y as i32 - dY));

    // drop the points that are p1 or p2 or outside the board
    let out: Vec<_> = antinodes
        .into_iter()
        // .filter(|&x| x != *p1 && x != *p2 && board.within(&x))
        .collect();

    // dbg!(&out);
    out
}
