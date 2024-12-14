mod util;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use util::{Board, Direction, Point};

type Map = Board;

#[derive(Debug, Clone, Eq)]
struct Face {
    a: Point,
    b: Point,
}
impl Face {}
// ensure they are equal regardless of order
impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b || self.a == other.b && self.b == other.a
    }
}
// TODO: this isn't quite right
impl Hash for Face {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let c = Point::new(self.a.x + self.b.x, self.a.y + self.b.y);
        c.hash(state);
    }
}

struct Region {
    points: HashSet<Point>,
    plant_type: char,
}
impl Region {
    fn area(&self) -> usize {
        self.points.len()
    }
    fn perimeter(&self) -> usize {
        let mut result = 0;
        // brute force approach
        for point in &self.points {
            // look in every direction,
            // each direction that is NOT in self.points
            // adds one to the perimeter
            for direction in Direction::all() {
                let new_point = direction.step(point);
                if !self.points.contains(&new_point) {
                    result += 1;
                }
            }
        }

        result
    }
    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }
    fn price2(&self) -> usize {
        self.area() * self.sides()
    }
    fn sides(&self) -> usize {
        // try this scannign algo:
        // https://www.reddit.com/r/adventofcode/comments/1hcxmpp/2024_day_12_part_2_visualisation_of_my_first/
        // Basically:
        // For each direction:
        //      Find all the squares with a "fence" in that direction (moving that way would leave the
        //      region)
        //      Add up the disjoint groups (anything touching counts as 1)
        //  add them all up
        fn count_disjoint(points: &Vec<&Point>) -> usize {
            let mut shared_faces: HashSet<Face> = HashSet::new();
            for p in points {
                // test if it has any existing groups it is adjacent to
                for g in points {
                    // if they are adjacent
                    if p.y == g.y && (p.x - g.x).abs() == 1 || p.x == g.x && (p.y - g.y).abs() == 1
                    {
                        shared_faces.insert(Face { a: **p, b: **g });
                    }
                }
            }

            points.len() - shared_faces.len()
        }

        let mut sides = 0;
        for direction in Direction::all() {
            let mut fence_points: Vec<&Point> = Vec::new();
            for point in &self.points {
                let test_point = direction.step(&point);
                if self.points.contains(&test_point) {
                    // not a fence
                } else {
                    fence_points.push(point);
                }
            }
            sides += count_disjoint(&fence_points);
        }

        sides
    }
}

fn main() {
    println!("AoC 2024: Day 12");

    // let lines = util::get_lines_from_file("input/day12-test.txt");
    let lines = util::get_lines_from_file("input/day12.txt");

    let map = Map { lines };

    let mut all_good_points: HashSet<Point> = HashSet::new();
    let mut regions: Vec<Region> = Vec::new();

    // iterate every point and visit it, collect the created regions
    for point in &map.all_points() {
        // skip any point that's alreaday in a region
        if !all_good_points.contains(point) {
            let current_type = map.get(point).unwrap();
            let mut good_points: HashSet<Point> = HashSet::new();
            visit(&mut good_points, &map, point, current_type);
            all_good_points.extend(&good_points);

            regions.push(Region {
                points: good_points,
                plant_type: current_type,
            });
        }
    }

    // println!("{}", regions.len());
    let total_price = regions.iter().map(|x| x.price()).sum::<usize>();
    println!("Part 1, total price: {}", total_price);

    let total_price2 = regions.iter().map(|x| x.price2()).sum::<usize>();
    println!("Part 2, total price: {}", total_price2);
}

fn visit(good_points: &mut HashSet<Point>, map: &Map, point: &Point, current_type: char) {
    if good_points.contains(point) {
        return;
    }
    if let Some(plant_type) = map.get(point) {
        // println!("visiting {} -> {}", point, plant_type);
        // if this node is good, continue searching
        if plant_type == current_type {
            good_points.insert(*point);
            for direction in Direction::all() {
                let new_point = direction.step(point);
                // println!("{new_point}");
                visit(good_points, map, &new_point, current_type);
            }
        }
    }
}
