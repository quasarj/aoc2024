mod util;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
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
    fn get_next_steps(&self, node: &Point) -> Vec<Point> {
        let adjacent_steps: Vec<_> = Direction::all()
            .iter()
            .map(|d| d.step(node))
            .collect();

        let available_steps: Vec<_> = adjacent_steps
            .into_iter()
            // exclude walls
            .filter(|p| self.get(p) != Some('#'))
            .collect();

        available_steps
    }
}

fn main() {
    println!("AoC 2024: Day 16");



    let lines = util::get_lines_from_file("input/day16-test.txt");
    // let lines = util::get_lines_from_file("input/day16.txt");


    let maze = Maze { lines };

    // // always start facing East, according to instructions
    // let start_direction = Direction::East;

    
    if let Some(path) = astar(&maze) {
        for y in 0..maze.height() {
            for x in 0..maze.width() {
                let point = Point::new(x as i32, y as i32);
                let val = maze.get(&point).unwrap();
                if path.contains(&point) {
                    print!("o");
                } else {
                    print!("{val}");
                }
            }
            println!();
        }
    }

}
fn direction_moving(current_node: &Point, came_from_node: &Point) -> Direction {
    if current_node.x < came_from_node.x {
        Direction::West
    } else if current_node.x > came_from_node.x {
        Direction::East
    } else if current_node.y < came_from_node.y {
        Direction::North
    } else if current_node.y > came_from_node.y {
        Direction::South
    } else {
        panic!("current and came_from node are the same? or don't touch?");
    }
}

fn astar(maze: &Maze) -> Option<Vec<Point>> {
    // simple distance, used to estimate cost
    fn heuristic(node: &Point, goal: &Point) -> i32 {
        (node.x - goal.x).abs() + (node.y - goal.y).abs()
    }
    fn rebuild_path(node: &Point, came_from: &HashMap<Point, Point>) -> Vec<Point> {
        let mut path = vec![*node];
        println!("Beginning at node={node}");
        let mut curr = node;
        loop {
            if !came_from.contains_key(&curr) {
                // we're done, should be back at the start now
                // dbg!(path);
                println!("Path length: {}", path.len());
                break;
            }

            curr = came_from.get(curr).unwrap();
            path.push(*curr);
        }

        path
    }

    let start_node = maze.get_start();
    let goal_node = maze.get_end();

    let mut open_set = BinaryHeap::new();
    open_set.push((Reverse(0), start_node));

    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut cost_so_far: HashMap<Point, i32> = HashMap::new();
    cost_so_far.insert(start_node, 0);


    loop {
        if let Some((curr_cost, curr_node)) = open_set.pop() {
            // println!("{curr_cost}, {curr_node}");
            if curr_node == goal_node {
                // technically don't need to do this, just need the cost
                return Some(rebuild_path(&goal_node, &came_from));
                // dbg!(cost_so_far.get(&goal_node));
            }

            // TODO determine the direction we are "going"

            // we should be in the came_from set, if not use initial direction
            let current_direction = if let Some(came_from_node) = came_from.get(&curr_node) {
                direction_moving(&curr_node, &came_from_node)
            } else {
                Direction::East
            };

            // TODO: update get_next_steps to return the cost as well,
            //       it will have to take current_direction as an arg
            for neighbor in maze.get_next_steps(&curr_node) {
                let new_cost = cost_so_far[&curr_node] + 1; // TODO update

                // TODO I don't like this at all, surely a better way
                // to adapt "if neighbor not in cost_so_far or new_cost < cost_so_far[neighbor]"
                let mut cont = false;
                if !cost_so_far.contains_key(&neighbor) {
                    cont = true;
                } else if new_cost < *cost_so_far.get(&neighbor).unwrap() {
                    cont = true;
                }
                if cont {
                    cost_so_far.insert(neighbor, new_cost);
                    let priority = new_cost + heuristic(&neighbor, &goal_node);
                    // remember to make in min-heap, have to reverse priority
                    open_set.push((Reverse(priority), neighbor));
                    came_from.insert(neighbor, curr_node);
                }
            }
        } else {
            break;
        }
    }

    None
}
