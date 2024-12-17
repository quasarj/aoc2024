mod util;
use std::collections::HashSet;
use std::cmp::Ordering;
use util::{Board, Direction, Point};

type Maze = Board;

impl Maze {
    fn get_start(&self) -> Point {
        let start = self
            .all_points()
            .into_iter()
            .filter(|x| self.get(x) == Some('S'))
            .next() // take only the first thing matched
            .expect("maze has no start?"); 
        start
    }
    fn get_end(&self) -> Point {
        let start = self
            .all_points()
            .into_iter()
            .filter(|x| self.get(x) == Some('E'))
            .next() // take only the first thing matched
            .expect("maze has no end?"); 
        start
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Path {
    steps: HashSet<Point>,
    last_step: Point,
}
impl Path {
    fn from_other(other: &Path, step: &Point) -> Self {
        let mut new_steps: HashSet<Point> = HashSet::new();
        new_steps.extend(&other.steps);
        new_steps.insert(*step);

        Path {
            steps: new_steps,
            last_step: *step
        }
    }
    fn empty(step: &Point) -> Self {
        let mut new_steps: HashSet<Point> = HashSet::new();
        new_steps.insert(*step);

        Path {
            steps: new_steps,
            last_step: *step
        }
    }
    fn get_next_steps(&self, maze: &Maze) -> Vec<Point> {
        let adjacent_steps: Vec<_> = Direction::all()
            .iter()
            .map(|d| d.step(&self.last_step))
            .collect();

        let available_steps: Vec<_> = adjacent_steps
            .into_iter()
            // exclude walls
            .filter(|p| maze.get(p) != Some('#'))
            // exclude if we've visited it already on this path
            .filter(|p| !self.steps.contains(p))
            .collect();

        available_steps
    }
}

// paths are ordered by their length
impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps.len().cmp(&other.steps.len())
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    println!("AoC 2024: Day 16");


    // let lines = util::get_lines_from_file("input/day16-test.txt");
    let lines = util::get_lines_from_file("input/day16.txt");


    let maze = Maze { lines };

    // // always start facing East, according to instructions
    // let start_direction = Direction::East;

    if let Some(solution) = solve(&maze) {
        println!("Solution found! {} steps", solution.steps.len());
    }
}
fn solve(maze: &Maze) -> Option<Path> {
    let start_pos = maze.get_start();
    let end_pos = maze.get_end();

    let mut candidates: Vec<Path> = Vec::new();
    candidates.push(Path::empty(&start_pos));
    loop {
        if candidates.len() == 0 {
            break;
        }
        candidates.sort();
        // easier to reverse than to pop off the top
        candidates.reverse();
        let candidate = candidates.pop().unwrap();

        for step in candidate.get_next_steps(&maze) {
            let new_path = Path::from_other(&candidate, &step);
            if step == end_pos {
                return Some(new_path);
            } else {
                candidates.push(new_path);
            }

        }
    }

    None
}
