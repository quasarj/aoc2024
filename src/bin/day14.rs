mod util;
use std::collections::HashSet;
use util::Point;

#[derive(Debug)]
struct Map {
    max_x: i32,
    max_y: i32,
}
#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Robot<'a> {
    position: Point,
    velocity: Velocity,
    map: &'a Map,
}

impl Point {
    fn step(&mut self, velocity: &Velocity, map: &Map) -> &Self {
        fn wrapping_add(a: i32, b: i32, max: i32) -> i32 {
            let mut res = a + b;
            if res < 0 {
                res = max + res;
            } else if res >= max {
                res = res - max;
            }

            res
        }
        self.x = wrapping_add(self.x, velocity.x, map.max_x);
        self.y = wrapping_add(self.y, velocity.y, map.max_y);

        self
    }
}

fn split_vals(v: &str) -> (i32, i32) {
    let r: Vec<i32> = v.split(',').map(|x| x.parse()).flatten().collect();

    (r[0], r[1])
}

fn print(robots: &Vec<Robot>, map: &Map) {
    let points: HashSet<_> = robots.iter().map(|r| r.position).collect();
    for i in 0..map.max_y {
        for j in 0..map.max_x {
            if points.contains(&Point { x: j, y: i}) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() {
    println!("AoC 2024: Day 14");

    // let lines = util::get_lines_from_file("input/day14-test.txt");
    // let map = Map { max_x: 11, max_y: 7 };

    let lines = util::get_lines_from_file("input/day14.txt");
    let map = Map { max_x: 101, max_y: 103 };


    let mut robots: Vec<Robot> = Vec::new();

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let p_part = parts[0];
        let v_part = parts[1];

        let p: Vec<_> = p_part.split('=').collect();
        let (p_x, p_y) = split_vals(p[1]);

        let v: Vec<_> = v_part.split('=').collect();
        let (v_x, v_y) = split_vals(v[1]);

        robots.push(Robot {
            position: Point { x: p_x, y: p_y },
            velocity: Velocity { x: v_x, y: v_y },
            map: &map,
        });

    }

    fn looks_like_a_tree(robots: &Vec<Robot>, map: &Map) -> bool {
        let map_center_x = map.max_x / 2;
        // if there are points all the way down the center

        let r: Vec<_> = robots
            .iter()
            .filter(|r| {
                r.position.x == map_center_x ||
                r.position.x == (map_center_x - 1) ||
                r.position.x == (map_center_x + 1)
            })
            .collect();
        if r.len() > 25 {
            true
        } else {
            false
        }
    }



    println!("0 - before we begin");
    print(&robots, &map);
    for seconds in 1..=6771 { // this is the answer to p2
        let mut print_ = false;
        for r in &mut robots {
            r.position.step(&r.velocity, r.map);
        }

        if looks_like_a_tree(&robots, &map) {
            println!("{seconds}");
            print(&robots, &map);
        }
    }

    let q1: Vec<_> = robots
        .iter()
        .filter(|r| {
            r.position.x < map.max_x / 2 &&
            r.position.y < map.max_y / 2
        })
        .collect();
    let q2: Vec<_> = robots
        .iter()
        .filter(|r| {
            r.position.x > map.max_x / 2 &&
            r.position.y < map.max_y / 2
        })
        .collect();
    let q3: Vec<_> = robots
        .iter()
        .filter(|r| {
            r.position.x < map.max_x / 2 &&
            r.position.y > map.max_y / 2
        })
        .collect();
    let q4: Vec<_> = robots
        .iter()
        .filter(|r| {
            r.position.x > map.max_x / 2 &&
            r.position.y > map.max_y / 2
        })
        .collect();

    // println!("Q1 count: {}", q1.len());
    // println!("Q2 count: {}", q2.len());
    // println!("Q3 count: {}", q3.len());
    // println!("Q4 count: {}", q4.len());

    let safety_factor = q1.len() * q2.len() * q3.len() * q4.len();
    println!("Part 1, saftey factor: {safety_factor}");


}
