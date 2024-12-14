mod util;
enum Rules {
    Zero,
    Even,
    Fallback,
}

type StoneInt = u64;

fn main() {
    println!("AoC 2024: Day 11");

    // let lines = util::get_lines_from_file("input/day11-test.txt");
    let lines = util::get_lines_from_file("input/day11.txt");

    // there should only be one input line

    let stones: Vec<_> = lines[0]
        .split_whitespace()
        .map(|x| x.parse::<StoneInt>().expect("stone didn't parse"))
        .collect();

    // dbg!(&stones);
    for i in 1..=25 {
        let j = test(stones.clone(), i);
        println!("{i}->{j}");
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
