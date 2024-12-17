mod util;
// use std::collections::HashSet;
use util::{Board, Direction, Point};

type Maze = Board;

fn main() {
    println!("AoC 2024: Day 16");

    let lines = util::get_lines_from_file("input/day16-test.txt");
    // let lines = util::get_lines_from_file("input/day16.txt");



    let maze = Maze { lines };
    dbg!(&maze);


    // idea 1:
    // recursive function "visit"
    // has to take input of: the direction we're going,
    //                       a vec of nodes we have visited on this path,
    //                       the node we are looking at now
    //
    // if the node/position is a wall, return None (this is a dead end)
    // if the node is E, return ??? (maybe 1?)
    // otherwise keep searching:
    //     add ourself to the end of the visited_nodes vec (new copy)
    //     collect:
    //          visit in the current direction + 1
    //          visit in each other direction + 1000 (turning penalty)
    //          except: if any of those directions are in the
    //                  list of nodes we have visited, don't visit it
    //
    //      if all come back as None, return None
    //      If any come back with a score, return the lowest score

    let start = maze
        .all_points()
        .into_iter()
        .filter(|x| maze.get(x) == Some('S'))
        .next() // take only the first thing matched
        .expect("maze has no start?"); 

    // always start facing East, according to instructions
    let start_direction = Direction::East;

    // initial call: TODO find the Start point first (hardcoded here)
    let r = visit(&maze, &start_direction, &Vec::new(), start);
    dbg!(r);

}

fn visit(maze: &Maze, heading: &Direction, visited: &Vec<Point>, pos: Point) -> Option<i32> {
    if let Some(val) = maze.get(&pos) {
        if val == '#' {
            return None; // dead-end reached
        }
        if val == 'E' { // found the end!
            return Some(0); 
        }
        let mut visited_ext = visited.clone();
        visited_ext.push(pos);
        let mut legs: Vec<i32> = Vec::new();

        for d in Direction::all() {
            let cost = if &d == heading { 1 } else { 1001 };
            let new_pos = d.step(&pos);
            if !visited_ext.contains(&new_pos) {
                if let Some(c) = visit(maze, &d, &visited_ext, new_pos) {
                    legs.push(c + cost);
                }
            }
        }
        // return the least costly leg
        let cheapest_leg = legs
            .iter()
            .min()
            .copied();
        dbg!(&cheapest_leg);
        cheapest_leg
    } else {
        None
    }
}
