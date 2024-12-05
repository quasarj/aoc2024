use regex::Regex;
mod util;

fn main() {
    println!("AoC 2024: Day 3");

    // let lines = util::get_lines_from_file("input/day3-test.txt");
    let lines = util::get_lines_from_file("input/day3.txt");

    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut total = 0;
    let mut active = true;
    let mut active_total = 0;

    for line in &lines {
        // Find all matches in the input string
        for capture in re.captures_iter(line) {
            if let Some(full_match) = capture.get(0) {
                // println!("Full match: {}", full_match.as_str());

                if let Some(_x_match) = capture.get(1) {
                    let x = capture.get(1).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
                    let y = capture.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());

                    total += x * y;
                    if active {
                        active_total += x * y;
                    }
                } else {
                    match full_match.as_str() {
                        "don't()" => active = false,
                        "do()" => active = true,
                        _ => {}
                    }
                }
            }
        }
    }

    println!("Part 1 total: {total}");
    println!("Part 2 total: {active_total}");
}
