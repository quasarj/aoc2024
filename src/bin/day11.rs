mod util;
use std::collections::HashMap;

enum Rules {
    Zero,
    Even,
    Fallback,
}

type StoneInt = u64;

struct StoneCounter {
    cache: HashMap<(StoneInt, usize), StoneInt>,
}
impl StoneCounter {
    fn new() -> Self {
        StoneCounter {
            cache: HashMap::new(),
        }
    }
    fn count(&mut self, stone: StoneInt, blink_count: usize) -> StoneInt {
        if blink_count == 0 {
            return 1;
        }

        let cache_key = (stone, blink_count);
        if !self.cache.contains_key(&cache_key) {
            // calculate it
            if stone == 0 {
                // change it to 1, count doesn't go up
                let res = self.count(1, blink_count - 1);
                self.cache.insert(cache_key, res);
            } else if even(stone) {
                // split it
                let (l, r) = split(stone);
                let res = self.count(l, blink_count - 1) + self.count(r, blink_count - 1);
                
                self.cache.insert(cache_key, res);
            } else {
                // multiply by 2024
                let res = self.count(stone * 2024, blink_count - 1);
                self.cache.insert(cache_key, res);
            }
        }
        *self.cache.get(&cache_key).unwrap()
    }
    // fn test(&mut self) {
    //     assert_eq!(self.count(0, 0), 1, "0 @ 0 = 0/1");
    //     assert_eq!(self.count(0, 1), 1, "0 @ 1 = 1/1");
    //     assert_eq!(self.count(0, 2), 1, "0 @ 2 = 2024/1");
    //     assert_eq!(self.count(0, 3), 2, "0 @ 3 = 20 24/2");
    //     assert_eq!(self.count(0, 4), 4, "0 @ 4 = 2 0 2 4/4");
    //     assert_eq!(self.count(0, 5), 4, "0 @ 5 = 4048 1 4048 8096/4");
    //     assert_eq!(self.count(0, 6), 7, "0 @ 6 = 40 48 2024 40 48 80 96/7");
    //     assert_eq!(self.count(0, 7), 14, "0 @ 7 = 4 0 4 8 20 24 4 0 4 8 8 0 9 6/14");
    // }
}


fn main() {
    println!("AoC 2024: Day 11");

    // let lines = util::get_lines_from_file("input/day11-test.txt");
    let lines = util::get_lines_from_file("input/day11.txt");

    let stones: Vec<_> = lines[0] // there should only be one line of input
        .split_whitespace()
        .map(|x| x.parse::<StoneInt>().expect("stone didn't parse"))
        .collect();

    let mut stonecounter = StoneCounter::new();

    // assert_eq!(stonecounter.count(0, 0), 1, "0 @ 0 = 0/1");
    // assert_eq!(stonecounter.count(0, 1), 1, "0 @ 1 = 1/1");
    // assert_eq!(stonecounter.count(0, 2), 1, "0 @ 2 = 2024/1");
    // assert_eq!(stonecounter.count(0, 3), 2, "0 @ 3 = 20 24/2");
    // assert_eq!(stonecounter.count(0, 4), 4, "0 @ 4 = 2 0 2 4/4");
    // assert_eq!(stonecounter.count(0, 5), 4, "0 @ 5 = 4048 1 4048 8096/4");
    // assert_eq!(stonecounter.count(0, 6), 7, "0 @ 6 = 40 48 2024 40 48 80 96/7");
    // assert_eq!(stonecounter.count(0, 7), 14, "0 @ 7 = 4 0 4 8 20 24 4 0 4 8 8 0 9 6/14");

    // total them
    for (part, blinks) in [(1, 25), (2, 75)] {
        let t = stones.iter().map(|s| stonecounter.count(*s, blinks)).sum::<StoneInt>();
        println!("Part {part} total: {t}");
    }

}
fn test(stones: Vec<StoneInt>, x: usize) -> usize {
    let mut current = stones;
    for i in 0..x {
        current = blink(&current);
    }
    current.len()
}
fn blink(stones: &Vec<StoneInt>) -> Vec<StoneInt> {
    // blink

    let mut output: Vec<StoneInt> = Vec::new();
    for stone in stones {
        // println!("{}", stone);
        // rule Zero
        if *stone == 0 {
            output.push(1);
        } else if even(*stone) {
            let (l, r) = split(*stone);
            output.push(l);
            output.push(r);
        } else {
            output.push(stone * 2024);
        }
    }

    output
}

fn even(stone: StoneInt) -> bool {
    let digits = stone.ilog10() + 1;
    if digits % 2 == 0 {
        true
    } else {
        false
    }
}

fn split(val: StoneInt) -> (StoneInt, StoneInt) {
    let digits = val.ilog10() + 1;
    let N = StoneInt::pow(10, digits / 2);
    let first_part = val / N;
    let second_part = val % N;

    // dbg!(N, &first_part, &second_part);

    (first_part, second_part)
}

fn concat(a: u64, b: u64) -> u64 {
    // how many digits is b?
    let digits = b.ilog10() + 1;

    // shift a left by that many
    let a_prime = a * (u64::pow(10, digits));

    a_prime + b
}
