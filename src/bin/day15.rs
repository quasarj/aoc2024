mod util;
// use std::collections::HashSet;
use util::{Board, Direction, Point};

type Map = Board;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct DWPoint {
    a: Point,
    b: Point,
}
impl DWPoint {
    fn from_single(x: i32, y: i32) -> Self {
        DWPoint {
            a: Point { x: x * 2, y },
            b: Point { x: x * 2 + 1, y },
        }
    }
    fn contains(&self, point: &Point) -> bool {
        self.a == *point || self.b == *point
    }
}

fn split_input(lines: Vec<String>) -> (Vec<String>, Vec<String>) {

    // this does copy the linput; it's difficult
    // to do this otherwise
    let mut map = lines.clone();
    for i  in 0..lines.len() {
        if lines[i] == "" {
            let mov = map.split_off(i);
            return (map, mov);
        }
    }
    
    panic!("Input was not valid");
}

fn find_robot(map: &Map) -> Point {
    let robots: Vec<_> = map
        .all_points()
        .into_iter()
        .filter(|p| map.get(p) == Some('@'))
        .collect();

    if robots.len() != 1 {
        panic!("Wrong number of robots found! (none, or more than one!)");
    }

    robots[0]
}

fn mov(pos: &Point, map: &mut Map, direction: &Direction) -> bool {
    // check this char, if it's a wall/barrier -> false
    // look at the point in the direction to move
    // if it is None we give up -> false
    // is it empty?
    //      yes: set it to `pos` char, set `pos` char to empty, -> true
    //      no: call mov on it in the same direction
    //         if that returns true, go ahead and set move (yes behavior) -> true
    //         if it returns false, just return false (can't move)

    let current_char = map
        .get(pos)
        .expect("tried to mov something that doesn't exist????");

    if current_char == '#' {
        // this is a wall/barrier, we cannot move it!
        return false;
    }

    let new_pos = direction.step(pos);
    if let Some(new_char) = map.get(&new_pos) {
        if new_char != '.' {
            if !mov(&new_pos, map, direction) {
                // can't be moved, we give up now
                return false;
            }
        }
        // ready to move
        map.set(&new_pos, current_char);
        map.set(pos, '.');

        return true;
    } else {
        // we're moving off the map, it seems
        return false;
    }
}


fn main() {
    println!("AoC 2024: Day 15");

    // let lines = util::get_lines_from_file("input/day15-test.txt");
    let lines = util::get_lines_from_file("input/day15.txt");

    let (map_lines, movement_lines) = split_input(lines);


    // Part 2

    // double the map dimensions TODO get rid of this
    let mut new_map_lines: Vec<String> = Vec::new();
    for line in &map_lines {
        let mut new_line: String = String::new();

        for c in line.chars() {
            let cc = match c {
                '#' => "##",
                'O' => "[]",
                '.' => "..",
                '@' => "@.",
                _ => todo!()
            };
            new_line.push_str(cc);
        }
        println!("{new_line} <- {line}");
        new_map_lines.push(new_line);
    }


    let mut barriers: Vec<DWPoint> = Vec::new();
    let mut boxes: Vec<DWPoint> = Vec::new();
    // there can be only one!
    let mut robots: Vec<Point> = Vec::new();

    for (y, line) in map_lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let dp = DWPoint::from_single(x as i32, y as i32);
            match c {
                '#' => barriers.push(dp),
                'O' => boxes.push(dp),
                '.' => { /* nothing to do here */}
                '@' => robots.push(Point::new(x as i32 * 2, y as i32)),
                _ => todo!()
            }
        }
    }
    assert_eq!(robots.len(), 1, "There can be only one robot!!");
    let mut robot = robots[0];
    dbg!(&robot);

    let map_height = map_lines.len();
    let map_width = map_lines[0].len() * 2;
    dbg!(map_height, map_width);

    fn print_big_map(height: usize, width: usize, barriers: &Vec<DWPoint>, boxes: &Vec<DWPoint>, robot: &Point) {
        for y in 0..height {
            for x in 0..width {
                let point = Point::new(x as i32, y as i32);
                if barriers
                    .iter()
                    .filter(|p| p.contains(&point))
                    .count() > 0 {
                    print!("#");
                } else if boxes
                    .iter()
                    .filter(|p| p.contains(&point))
                    .count() > 0 {
                    print!("|");
                } else if point == *robot {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
    print_big_map(map_height, map_width, &barriers, &boxes, &robot);


    fn canmove(point: &DWPoint, direction: &Direction, barriers: &Vec<DWPoint>, boxes: &Vec<DWPoint>) -> bool {
        // println!("canmove({point:?}, {direction:?}");
        // Here we are only testing to see if it would be POSSIBLE to move
        // in this direction. The actual move will be done elsewhere

        let a = direction.step(&point.a);
        let b = direction.step(&point.b);

        // do the new points intersect any barriers?
        let obstacles = barriers
            .iter()
            .filter(|p| p.contains(&a) || p.contains(&b))
            .count();
        if obstacles > 0 {
            return false;
        }
        
        // are there any other boxes in the way that we need to move first?
        let other_boxes: Vec<_> = boxes
            .iter()
            // it's not us, but we would intersect it
            .filter(|p| p != &point && (p.contains(&a) || p.contains(&b)))
            .collect();

        // if all the other boxes can move, then we can move
        // NOTE: if other_boxes is empty, this returns true (can move)
        other_boxes
            .iter()
            .all(|x| canmove(x, direction, barriers, boxes))
    }

    fn movbig(point: &DWPoint, direction: &Direction, boxes: &mut Vec<DWPoint>) -> DWPoint {
        // We assume it has already been determined that moving is possible!
        // No barriers are in the way
        let a = direction.step(&point.a);
        let b = direction.step(&point.b);

        // are there any other boxes in the way that we need to move first?
        let boxes_copy = boxes.clone();
        let other_boxes: Vec<_> = boxes_copy
            .iter()
            // it's not us, but we would intersect it
            .filter(|p| p != &point && (p.contains(&a) || p.contains(&b)))
            .collect();

        // if other_boxes.len() > 0 {
        //     println!("Can't move yet because {} boxes are in the way", other_boxes.len());
        //     println!("my pos {point:?}");
        // }

        for ob in other_boxes {
            // println!("moving other box first");
            movbig(&ob, direction, boxes);
        }

        // actually move it
        for i in 0..boxes.len() {
            if boxes[i] == *point {
                boxes[i].a = a;
                boxes[i].b = b;
            }
        }

        DWPoint { a, b }
    }

    fn movrob(robot: &Point, direction: &Direction, barriers: &Vec<DWPoint>, boxes: &mut Vec<DWPoint>) -> Option<Point> {
        // println!("Robot asked to move in {direction:?}");
        let a = direction.step(&robot);

        // do the new points intersect any barriers?
        if barriers
            .iter()
            .filter(|p| p.contains(&a))
            .count() > 0 {
            return None;
        }
        
        // are there any other boxes in the way that we need to move first?
        let boxes_copy = boxes.clone();
        let boxes_in_the_way: Vec<_> = boxes_copy
            .iter()
            // it's not us, but we would intersect it
            .filter(|p| p.contains(&a))
            .collect();

        // println!("There are {} boxes in the way, checking to see if we can move them", boxes_in_the_way.len());
        if !(boxes_in_the_way
            .iter()
            .all(|x| canmove(x, direction, barriers, boxes))) {
            // the boxes in the way can't move
            return None;
        }

        // println!("Now attempting to actually move those boxes");

        // move any boxes out of the way first
        for ob in boxes_in_the_way {
            // println!("moving a box first, before robot move");
            movbig(&ob, direction, boxes);
        }

        Some(a)
    }

    // let dir = Direction::East;


    // if let Some(rb) = movrob(&robot, &dir, &barriers, &mut boxes) {
    //     println!("Robot was able to move!");
    //     robot = rb;
    // }



    for line in &movement_lines {
        for c in line.chars() {
            let direction = match c {
                '^' => Direction::North,
                'v' => Direction::South,
                '>' => Direction::East,
                '<' => Direction::West,
                _ => { panic!("what is this direction {c}???") }
            };
            if let Some(rb) = movrob(&robot, &direction, &barriers, &mut boxes) {
                // println!("Robot was able to move!");
                robot = rb;
            }
        }
    }
    print_big_map(map_height, map_width, &barriers, &boxes, &robot);

    let p2_score: i32 = boxes
        .iter()
        .map(|b| 100 * b.a.y + b.a.x)
        .sum();



    // Part 1
    let mut map = Map { lines: map_lines };

    let mut robot = find_robot(&mut map);

    dbg!(&map);
    for line in movement_lines {
        for c in line.chars() {
            let direction = match c {
                '^' => Direction::North,
                'v' => Direction::South,
                '>' => Direction::East,
                '<' => Direction::West,
                _ => { panic!("what is this direction {c}???") }
            };
            if mov(&robot, &mut map, &direction) {
                // move the robot pointer
                robot = direction.step(&robot);
            }
        }
    }
    dbg!(&map);

    // calculate the final score
    let score: i32 = map.all_points()
        .iter()
        .filter(|p| map.get(p) == Some('O'))
        .map(|p| 100 * p.y + p.x)
        .sum();

    println!("Part 1: {score}");
    println!("Part 2: {p2_score}");

}
